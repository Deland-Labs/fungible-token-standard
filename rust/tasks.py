import json
import os

from invoke import task
import shutil;
import os;

canister_ids_dir = "canister_ids"
package_dir = "package"


def get_envs():
    # get file names
    file_names = os.listdir(canister_ids_dir)
    # get file names without extension
    envs = [os.path.splitext(file_name)[0] for file_name in file_names]
    return envs


@task
def package(ctx):
    # build package with dfx build
    ctx.run('dfx build')

    # load dfx.json as json
    with open('dfx.json') as f:
        dfx = json.load(f)

    # get canister names
    canisters_node = dfx['canisters']
    # get keys of canisters
    canisters_names = list(canisters_node.keys())
    print(canisters_names)
    # load candid as value and canister name as a key into a map from canisters_node
    canister_dids = {canister_name: canister_node['candid'] for canister_name, canister_node in canisters_node.items()}
    print(canister_dids)

    # reset package dir
    ctx.run(f"rm -rf {package_dir}")
    # create package dir
    ctx.run(f"mkdir {package_dir}")


    envs = get_envs()
    print(envs)

    release_base_dir = "target/wasm32-unknown-unknown/release"

    # check file size of each wasm file must be less than 2MB
    print("Checking file size of each wasm file must be less than 2MB")
    check_file_size = True
    if (check_file_size):
        for canister_name in canisters_names:
            opt_wasm_file = f"{release_base_dir}/{canister_name}_opt.wasm"
            wasm_file = f"{release_base_dir}/{canister_name}.wasm"
            if (os.path.isfile(opt_wasm_file)):
                # replace file with opt file
                shutil.copyfile(opt_wasm_file, wasm_file)
            # get file size
            file_size = ctx.run(f"stat -c %s {wasm_file}", hide=True).stdout.strip()
            # print file size in MB
            print(f"{canister_name} file size: {int(file_size) / 1024 / 1024} MB")
            # check file size
            if int(file_size) > 2 * 1024 * 1024:
                raise Exception(f"{wasm_file} is too large")
        print("All wasm files are less than 2MB")

    # create dfx.json
    print("Creating dfx.json")
    dfx_json = {
        "defaults": {
            "build": {
                "args": "",
                "packtool": ""
            }
        },
        "networks": {
            "local": {
                "bind": "127.0.0.1:8000",
                "type": "ephemeral"
            },
            "ic": {
                "providers": ["https://ic0.app"],
                "type": "persistent"
            },
        },
        "version": 1
    }
    # create canisters node
    canisters_node = {}
    for canister_name in canisters_names:
        canisters_node[canister_name] = {
            "candid": f"assets/{canister_name}.did",
            "wasm": f"assets/{canister_name}.wasm",
            "build": [],
            "type": "custom"
        }

    # add canisters node to dfx_json
    dfx_json['canisters'] = canisters_node

    # create package for each env
    print("Creating package for each env")
    for env in envs:
        # create package dir for env
        ctx.run(f"mkdir {package_dir}/{env}")
        # create assets dir for env
        ctx.run(f"mkdir {package_dir}/{env}/assets")
        ctx.run(f"cp {canister_ids_dir}/{env}.json {package_dir}/{env}/canister_ids.json")
        # copy release canister files to package dir
        for canister_name in canisters_names:
            ctx.run(f"cp {release_base_dir}/{canister_name}.wasm {package_dir}/{env}/assets/{canister_name}.wasm")
        # copy did to package dir
        for canister_name, canister_did in canister_dids.items():
            ctx.run(f"cp {canister_did} {package_dir}/{env}/assets/{canister_name}.did")
        # write dfx_json to dfx.json
        with open(f"{package_dir}/{env}/dfx.json", 'w') as f:
            json.dump(dfx_json, f, indent=2)


@task(package)
def package_zip(ctx):
    # create zip file for each env
    envs = get_envs()
    
    for env in envs:
        # create zip file with shutil
        shutil.make_archive(f"{package_dir}/{env}", 'zip', f"{package_dir}/{env}")
        print(f"{package_dir}/{env}.zip created")