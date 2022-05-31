# get first argument as version
VERSION=$1
# error if no version
if [ -z "$VERSION" ]; then
  echo "Error: No version specified"
  exit 1
fi
npx ic pack \
    --package-scope deland-labs \
    --canister-env-name CANISTER_ENV \
    --production-canister-env production \
    --publish \
    --package-version $VERSION