export const idlFactory = ({ IDL }) => {
  return IDL.Service({
    'notificationCount' : IDL.Func([], [IDL.Nat64], ['query']),
    'onTokenReceived' : IDL.Func([IDL.Nat, IDL.Text, IDL.Nat], [], []),
  });
};
export const init = ({ IDL }) => { return []; };
