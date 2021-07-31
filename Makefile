.PHONY: all
all: build

.PHONY: build
.SILENT: build
build:
	dfx canister --no-wallet create --all
	dfx build

.PHONY: install
.SILENT: install
install: build
	dfx canister --no-wallet  install --all

.PHONY: upgrade
.SILENT: upgrade
upgrade: build
	dfx canister --no-wallet  install --all --mode=reinstall

.PHONY: test
.SILENT: test
test: upgrade
		$(eval graphql_id := $(shell dfx canister id graphql))
		$(eval dft_id := $(shell dfx canister id dft))
		$(eval owner_id := $(shell dfx identity get-principal))
		dfx canister call graphql set_token_canister_id '(principal "$(dft_id)")'
		dfx canister call dft  setStorageCanisterID '(principal "$(graphql_id)")'
		dfx canister call dft  initialize '("Deland Token","DLD",18:nat8,100000000000000000000000000:nat)'
		dfx canister call dft  meta | grep 'Deland Token' && echo 'PASS'

		dfx canister call dft transfer '(null,"rrkah-fqaaa-aaaaa-aaaaq-cai",1000000000000000000:nat,null)'
		dfx canister call dft balanceOf "rrkah-fqaaa-aaaaa-aaaaq-cai"| grep '1_000_000_000_000_000_000' && echo 'PASS'

		dfx canister call dft approve '(null,"rrkah-fqaaa-aaaaa-aaaaq-cai",3000000000000000000:nat,null)'
		dfx canister call dft allowance '("$(owner_id)","rrkah-fqaaa-aaaaa-aaaaq-cai")'| grep '3_000_000_000_000_000_000' && echo 'PASS'
		dfx canister call dft transferFrom '(null,"$(owner_id)","rrkah-fqaaa-aaaaa-aaaaq-cai",1000000000000000000:nat)' \
		|grep 'Ok'' && echo 'PASS'

		dfx canister call graphql  graphql_query '("query { readTx { id,txid,txtype,caller,from,to,value,fee,timestamp} }", "{}")' \
		|grep '"txid":"2"'' && echo 'PASS'

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx
