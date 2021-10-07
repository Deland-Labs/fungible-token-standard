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
	dfx canister --no-wallet  install dft_motoko \
	  --argument '(null ,null ,"Deland Token", "DLD", 18:nat8, 100000000000000000000000000:nat, record { lowest = 1 : nat; rate = 0 : nat }, null)'

.PHONY: upgrade
.SILENT: upgrade
upgrade: build
	dfx canister --no-wallet  install dft_motoko \
	  --argument '(null ,null ,"Deland Token", "DLD", 18:nat8, 100000000000000000000000000:nat, record { lowest = 1 : nat; rate = 0 : nat }, null)' \
		--mode reinstall
 
define test_token_impl
	@echo "calling $(0), will test dft_motoko"
	$(eval dft_id := $(shell dfx canister id dft_motoko))
	$(eval owner_id := $(shell dfx identity get-principal))
	dfx canister call dft_motoko  name | grep 'Deland Token' && echo 'PASS name check'
	dfx canister call dft_motoko  symbol | grep 'DLD' && echo 'PASS symbol check'
	dfx canister call dft_motoko  decimals | grep '(18 : nat8)' && echo 'PASS decimals check'
	dfx canister call dft_motoko  totalSupply \
	| grep '(100_000_000_000_000_000_000_000_000 : nat)' && echo 'PASS totalSupply check'
	dfx canister call dft_motoko  fee | grep '0 : nat' && echo 'PASS fee check'
	dfx canister call dft_motoko  meta | grep 'Deland Token' && echo 'PASS meta check'

	dfx canister call dft_motoko transfer '(null,"rrkah-fqaaa-aaaaa-aaaaq-cai",1000000000000000000:nat,null)' \
	| grep 'Ok' && echo 'PASS transfer check'
	dfx canister call dft_motoko balanceOf "rrkah-fqaaa-aaaaa-aaaaq-cai" \
	| grep '1_000_000_000_000_000_000' && echo 'PASS balanceOf check'

	dfx canister call dft_motoko balanceOf "$(owner_id)" \
	| grep '99_999_999_000_000_000_000_000_000' && echo 'PASS balanceOf check'

	dfx canister call dft_motoko setExtend '(vec {record {k = "OFFICIAL_SITE"; v = "http://test.com" }})' \
	| grep '(true)' && echo 'PASS updateExtend test'
	dfx canister call dft_motoko extend  \
	| grep 'k = "OFFICIAL_SITE"; v = "http://test.com"' && echo 'PASS extend test'

	dfx canister call dft_motoko setFee '(record { lowest = 1 : nat; rate = 0 : nat })' \
	| grep '(true)' && echo 'PASS set fee test'
	dfx canister call dft_motoko fee  \
	| grep '(record { rate = 0 : nat; lowest = 1 : nat })' && echo 'PASS fee check 2'

	dfx canister call dft_motoko approve '(null,"rrkah-fqaaa-aaaaa-aaaaq-cai",3000000000000000000:nat,null)'
	dfx canister call dft_motoko allowance '("$(owner_id)","rrkah-fqaaa-aaaaa-aaaaq-cai")' \
	| grep '3_000_000_000_000_000_000' && echo 'PASS allowance check'
	sleep 3
endef

.PHONY: test
.SILENT: test
test_motoko: upgrade
	$(call test_token_impl,dft)

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx
