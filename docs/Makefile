TYPST_PATH ?= $(shell which typst)
OUTPUT_DIR ?= build
PDF_OUTPUT_FNAME ?= "PocketBeagle® 2 Coding Workshop.pdf"
PDF_OUTPUT_PATH = ${OUTPUT_DIR}/${PDF_OUTPUT_FNAME}

.PHONY: watch
watch:
	$(info "Watch pdf")
	$(TYPST_PATH) watch --font-path fonts main.typ ${PDF_OUTPUT_PATH}

.PHONY: build
build:
	$(info "Building pdf")
	mkdir -p ${OUTPUT_DIR}
	$(TYPST_PATH) compile --font-path fonts main.typ ${PDF_OUTPUT_PATH}
