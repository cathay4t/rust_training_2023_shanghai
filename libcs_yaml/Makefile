ROOT_DIR:=$(shell dirname $(realpath $(firstword $(MAKEFILE_LIST))))
VERSION=$(shell cat $(ROOT_DIR)/VERSION)
VERSION_MAJOR=$(shell echo $(VERSION) | cut -f1 -d.)
VERSION_MINOR=$(shell echo $(VERSION) | cut -f2 -d.)
VERSION_MICRO=$(shell echo $(VERSION) | cut -f3 -d.)
CLIB_SO_DEV=libcs_yaml.so
CLIB_SO_MAN=$(CLIB_SO_DEV).$(VERSION_MAJOR)
CLIB_SO_FULL=$(CLIB_SO_DEV).$(VERSION)
CLIB_HEADER=src/cs_yaml.h
CLIB_SO_DEV_RELEASE=../target/release/$(CLIB_SO_DEV)
CLIB_SO_DEV_DEBUG=../target/debug/$(CLIB_SO_DEV)
SRV_NAME=cs_yamld
SRV_PATH=../target/debug/cs_yamld

.PHONY: debug
debug:
	cargo build
	$(eval TMPDIR := $(shell mktemp -d))
	cp $(CLIB_SO_DEV_DEBUG) $(TMPDIR)/$(CLIB_SO_FULL)
	ln -sfv $(CLIB_SO_FULL) $(TMPDIR)/$(CLIB_SO_MAN)
	ln -sfv $(CLIB_SO_FULL) $(TMPDIR)/$(CLIB_SO_DEV)
	cp $(CLIB_HEADER) $(TMPDIR)/$(shell basename $(CLIB_HEADER))
	cc -g -Wall -Wextra -L$(TMPDIR) -I$(TMPDIR) \
		-o $(TMPDIR)/cs_yaml_test tests/cs_yaml_test.c -lcs_yaml
	$(SRV_PATH) &
	sleep 1
	LD_LIBRARY_PATH=$(TMPDIR) \
		$(TMPDIR)/cs_yaml_test
	rm -rf $(TMPDIR)
	pkill $(SRV_NAME)


$(CLIB_SO_DEV_DEBUG): debug

clib: $(CLIB_HEADER)

.PHONY: clib_check
clib_check: $(CLIB_SO_DEV_DEBUG) $(CLIB_HEADER)
	$(eval TMPDIR := $(shell mktemp -d))
	cp $(CLIB_SO_DEV_DEBUG) $(TMPDIR)/$(CLIB_SO_FULL)
	ln -sfv $(CLIB_SO_FULL) $(TMPDIR)/$(CLIB_SO_MAN)
	ln -sfv $(CLIB_SO_FULL) $(TMPDIR)/$(CLIB_SO_DEV)
	cp $(CLIB_HEADER) $(TMPDIR)/$(shell basename $(CLIB_HEADER))
	cc -g -Wall -Wextra -L$(TMPDIR) -I$(TMPDIR) \
		-o $(TMPDIR)/cs_yaml_test tests/cs_yaml_test.c -lcs_yaml
	$(SRV_PATH) &
	sleep 1
	LD_LIBRARY_PATH=$(TMPDIR) \
		valgrind --trace-children=yes --leak-check=full \
		--error-exitcode=1 \
		$(TMPDIR)/cs_yaml_test 1>/dev/null
	rm -rf $(TMPDIR)
	pkill $(SRV_PATH)

check: clib_check

clean:
	- cargo clean
	- rm -f ../target/debug/$(CLIB_SO_MAN)
	- rm -f ../target/debug/$(CLIB_SO_FULL)
