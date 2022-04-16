export const idlFactory = ({ IDL }) => {
  return IDL.Service({
    'onTokenReceived' : IDL.Func([IDL.Text, IDL.Nat], [IDL.Bool], []),
  });
};
export const init = ({ IDL }) => { return []; };
