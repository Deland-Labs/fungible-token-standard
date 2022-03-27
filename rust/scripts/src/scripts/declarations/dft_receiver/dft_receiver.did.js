export const idlFactory = ({ IDL }) => {
  const TokenHolder = IDL.Variant({
    'None' : IDL.Null,
    'Account' : IDL.Text,
    'Principal' : IDL.Principal,
  });
  return IDL.Service({
    'onTokenReceived' : IDL.Func([TokenHolder, IDL.Nat], [IDL.Bool], []),
  });
};
export const init = ({ IDL }) => { return []; };
