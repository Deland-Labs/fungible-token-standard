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
	dfx canister --no-wallet  install dft_rs \
	  --argument '(null ,null ,"Deland Token", "DLD", 18:nat8, 100000000000000000000000000:nat, record { lowest = 1 : nat; rate = 0 : nat } , null)'

.PHONY: upgrade
.SILENT: upgrade
upgrade: build
	dfx canister --no-wallet  install dft_rs \
	  --argument '(null ,null ,"Deland Token", "DLD", 18:nat8, 100000000000000000000000000:nat, record { lowest = 1 : nat; rate = 0 : nat }, null)' \
	  --mode reinstall
 
define test_token_impl
	@echo "calling $(0), will test dft"
	$(eval dft_id := $(shell dfx canister id dft_rs))
	$(eval owner_id := $(shell dfx identity get-principal))
	dfx canister call dft_rs  name | grep 'Deland Token' && echo 'PASS name check'
	dfx canister call dft_rs  symbol | grep 'DLD' && echo 'PASS symbol check'
	dfx canister call dft_rs  decimals | grep '(18 : nat8)' && echo 'PASS decimals check'
	dfx canister call dft_rs  totalSupply \
	| grep '(100_000_000_000_000_000_000_000_000 : nat)' && echo 'PASS totalSupply check'
	dfx canister call dft_rs  fee | grep '0 : nat' && echo 'PASS fee check'
	dfx canister call dft_rs  meta | grep 'Deland Token' && echo 'PASS meta check'

	dfx canister call dft_rs transfer '(null,"rrkah-fqaaa-aaaaa-aaaaq-cai",1000000000000000000:nat,null)' \
	| grep 'Ok' && echo 'PASS transfer check'
	dfx canister call dft_rs balanceOf "rrkah-fqaaa-aaaaa-aaaaq-cai" \
	| grep '1_000_000_000_000_000_000' && echo 'PASS balanceOf check'

	dfx canister call dft_rs balanceOf "$(owner_id)" \
	| grep '99_999_999_000_000_000_000_000_000' && echo 'PASS balanceOf check'

	dfx canister call dft_rs setExtend '(vec {record {k = "OFFICIAL_SITE"; v = "http://test.com" }})' \
	| grep '(true)' && echo 'PASS updateExtend test'
	dfx canister call dft_rs extend  \
	| grep 'k = "OFFICIAL_SITE"; v = "http://test.com"' && echo 'PASS extend test'

	dfx canister call dft_rs setFee '(record { lowest = 1 : nat; rate = 0 : nat })' \
	| grep '(true)' && echo 'PASS set fee test'
	dfx canister call dft_rs fee  \
	| grep '(record { rate = 0 : nat; lowest = 1 : nat })' && echo 'PASS fee check 2'

	dfx canister call dft_rs approve '(null,"rrkah-fqaaa-aaaaa-aaaaq-cai",3000000000000000000:nat,null)'
	dfx canister call dft_rs allowance '("$(owner_id)","rrkah-fqaaa-aaaaa-aaaaq-cai")' \
	| grep '3_000_000_000_000_000_000' && echo 'PASS allowance check'
	sleep 3
endef

.PHONY: test
.SILENT: test
test: upgrade
	$(call test_token_impl)

.PHONY: clean
.SILENT: clean
clean:
	rm -fr .dfx
