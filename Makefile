# The directory to move files into as part of the installation procedure.
DESTDIR :=

# The base directory for files at runtime.
PREFIX := /usr

ifneq (${DESTDIR},)
	INSTALL_PREFIX = ${DESTDIR}${PREFIX}
else
	INSTALL_PREFIX = ${PREFIX}
endif

INSTALL_BIN := ${INSTALL_PREFIX}/bin/ivm
INSTALL_LICENSE := ${INSTALL_PREFIX}/share/licenses/ivm/LICENSE

# The version of ivm, obtained from Cargo.toml.
VERSION != cargo pkgid | cut -d\# -f2 | cut -d: -f2

build:
	cargo build --release

install: ${INSTALL_BIN} ${INSTALL_LICENSE}

${INSTALL_BIN}:
	mkdir -p "$$(dirname ${@})"
	install -m755 target/release/ivm "${@}"

${INSTALL_LICENSE}:
	mkdir -p "$$(dirname ${@})"
	install -m644 LICENSE "${@}"

release/versions:
	ruby scripts/update_versions.rb ${VERSION}

release/changelog:
	ruby scripts/changelog.rb "${VERSION}"

release/commit:
	git commit Cargo.toml Cargo.lock CHANGELOG.md -m "Release v${VERSION}"
	git push origin "$$(git rev-parse --abbrev-ref HEAD)"

release/tag:
	git tag -s -a -m "Release v${VERSION}" "v${VERSION}"
	git push origin "v${VERSION}"

release/publish: release/versions release/changelog release/commit release/tag
	cargo publish

.PHONY: build release/changelog release/commit release/tag release/publish
