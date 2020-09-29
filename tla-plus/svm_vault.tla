----------------------------- MODULE svm_vault -----------------------------
EXTENDS TLC, Integers, Sequences, FiniteSets
          

CONSTANTS MASTERS, ORD_MASTERS, TOTAL_COINS, TOTAL_STEPS

ASSUME TOTAL_STEPS \in 1..30
ASSUME TOTAL_COINS \in 1..30

\* Operators

ACCOUNT_COUNT == Cardinality(MASTERS)

Range(f) == {f[x]: x \in DOMAIN f}

OrderSet(S) == CHOOSE seq \in [1..Cardinality(S) -> S]: Range(seq) = S

Ord(m) == CHOOSE i \in 1..Cardinality(MASTERS): ORD_MASTERS[i] = m

PairSet(seq) == {seq[1], seq[2]} 

RECURSIVE SumAccounts(_)
SumAccounts(f) == IF DOMAIN f = {}
                 THEN 0
                 ELSE 
                    LET x0 == RandomElement(DOMAIN f)
                        X == (DOMAIN f) \ {x0}
                        g == [x \in X |-> f[x]]
                    IN f[x0] + SumAccounts(g)
                                                         
AccountBalance == LET All == [MASTERS -> 0..TOTAL_COINS]
                  IN {f \in All: SumAccounts(f) = TOTAL_COINS}

VaultMasters[D \in (SUBSET MASTERS \ {{}})] ==
    LET All == [D -> MASTERS \X MASTERS \X MASTERS]
    IN
    {      
        f \in All:
            \A w \in DOMAIN f:
                \E x, y, z \in MASTERS:
                    f[w] = <<x, y, z>> /\ (Ord(w) > Ord(x)) /\ (Ord(x) > Ord(y)) /\ (Ord(y) > Ord(z))
    }


(*--algorithm Vault 
variables
    steps,
    acc_balance \in AccountBalance,
    vault_masters \in VaultMasters[{"M4", "M6"}],   
    vault_withdraw = [v \in Vaults |-> [Master |-> {}, Amount |-> 0]],
    vault_cancel_master = [v \in Vaults |-> {}];

define
    Vaults == DOMAIN vault_masters
    
    IsVaultAccount(v) == v \in Vaults
     
    IsPendingWithdraw(v) == Cardinality(vault_withdraw[v].Master) > 0  
    
    CanCancelWithdraw(v) == Cardinality(vault_cancel_master[v]) = 1 
    
    MultiSigBegin(v, m) == /\ IsVaultAccount(v)
                           /\ (~IsPendingWithdraw(v))
                           /\ (m \in PairSet(vault_masters[v]))
    
    MultiSigComplete(v, m) == /\ IsVaultAccount(v)
                              /\ IsPendingWithdraw(v)
                              /\ (m \in PairSet(vault_masters[v])) 
                              /\ (m \notin vault_withdraw[v].Master)         
                              
    SameTotalCoins == SumAccounts(acc_balance) = TOTAL_COINS
    
    NonNegBalanceInvariant == \A m \in MASTERS: acc_balance[m] >= 0
end define; 

procedure Transfer(from, to, delta) 
begin
    TransferLabel:
        assert from /= to;
        
        if (acc_balance[from] >= delta) /\ (delta > 0) then         
            FundSrc: 
                acc_balance[from] := acc_balance[from] - delta;
            FundDst:
                acc_balance[to] := acc_balance[to] + delta;  
                
            return; 
        else
            return;
        end if;
end procedure;

procedure ResetVault(vault) 
begin
    ResetLabel:
        vault_cancel_master[vault] := {};
        vault_withdraw[vault] := [Master |-> {}, Amount |-> 0]; 
    return;
end procedure;

procedure Withdraw(vault, master, to, delta) 
begin
    WithdrawLabel:
        assert MultiSigComplete(vault, master); 
        call Transfer(vault, to, delta);
        
        WithdrawResetLabel:
            call ResetVault(vault);
    return;
end procedure;

procedure QueuePendingWithraw(vault, master, delta)
begin
    QueueLabel:
        assert MultiSigBegin(vault, master); 
        assert delta > 0;
        
        vault_withdraw[vault] := [Master |-> {master}, Amount |-> delta];
    return;
end procedure;

begin   
    Start:
        steps := 0;

    Work:
    while steps < TOTAL_STEPS do
        steps := steps + 1;    
        
        assert SameTotalCoins; 
    
        either
            \* Fund 
            with s \in {m \in MASTERS: acc_balance[m] > 0},
                 d \in (MASTERS \ {s}),
                 a \in 1..acc_balance[s]
            do
               call Transfer(s, d, a);                  
            end with;        
        or
            \* Withdraw
            with v \in Vaults do       
                if ~IsPendingWithdraw(v) then
                    with m \in PairSet(vault_masters[v]), 
                         c \in 0..acc_balance[v] 
                    do
                        if c > 0 then
                            call QueuePendingWithraw(v, m, c);
                        end if;
                    end with;     
                else                   
                    with m \in (PairSet(vault_masters[v]) \ vault_withdraw[v].Master)
                    do   
                        call Withdraw(
                            v,  
                            m, 
                            RandomElement(vault_withdraw[v].Master),
                            vault_withdraw[v].Amount
                        );
                    end with;           
                end if;   
            end with;
        or
            \* Cancel a Pending Withdraw             
            with v \in Vaults do
                
                \* Cancellation makes no sense if there's no pending withdraw
                if ~IsPendingWithdraw(v) then
                    skip;
                else   
                    with m \in (PairSet(vault_masters[v]) \ vault_cancel_master[v]) do
                        vault_cancel_master[v] := vault_cancel_master[v] \union {m};

                        if CanCancelWithdraw(v) then
                            call ResetVault(v);
                        end if;            
                    end with;
                end if;
            end with;         
        end either;
    end while;
end algorithm;*)

\* BEGIN TRANSLATION - the hash of the PCal code: PCal-1a7b69ffcbf806fb7e5a338cc4866926
\* Parameter to of procedure Transfer at line 76 col 26 changed to to_
\* Parameter delta of procedure Transfer at line 76 col 30 changed to delta_
\* Parameter vault of procedure ResetVault at line 93 col 22 changed to vault_
\* Parameter vault of procedure Withdraw at line 101 col 20 changed to vault_W
\* Parameter master of procedure Withdraw at line 101 col 27 changed to master_
\* Parameter delta of procedure Withdraw at line 101 col 39 changed to delta_W
CONSTANT defaultInitValue
VARIABLES steps, acc_balance, vault_masters, vault_withdraw, 
          vault_cancel_master, pc, stack

(* define statement *)
Vaults == DOMAIN vault_masters

IsVaultAccount(v) == v \in Vaults

IsPendingWithdraw(v) == Cardinality(vault_withdraw[v].Master) > 0

CanCancelWithdraw(v) == Cardinality(vault_cancel_master[v]) = 1

MultiSigBegin(v, m) == /\ IsVaultAccount(v)
                       /\ (~IsPendingWithdraw(v))
                       /\ (m \in PairSet(vault_masters[v]))

MultiSigComplete(v, m) == /\ IsVaultAccount(v)
                          /\ IsPendingWithdraw(v)
                          /\ (m \in PairSet(vault_masters[v]))
                          /\ (m \notin vault_withdraw[v].Master)

SameTotalCoins == SumAccounts(acc_balance) = TOTAL_COINS

NonNegBalanceInvariant == \A m \in MASTERS: acc_balance[m] >= 0

VARIABLES from, to_, delta_, vault_, vault_W, master_, to, delta_W, vault, 
          master, delta

vars == << steps, acc_balance, vault_masters, vault_withdraw, 
           vault_cancel_master, pc, stack, from, to_, delta_, vault_, vault_W, 
           master_, to, delta_W, vault, master, delta >>

Init == (* Global variables *)
        /\ steps = defaultInitValue
        /\ acc_balance \in AccountBalance
        /\ vault_masters \in VaultMasters[{"M4", "M6"}]
        /\ vault_withdraw = [v \in Vaults |-> [Master |-> {}, Amount |-> 0]]
        /\ vault_cancel_master = [v \in Vaults |-> {}]
        (* Procedure Transfer *)
        /\ from = defaultInitValue
        /\ to_ = defaultInitValue
        /\ delta_ = defaultInitValue
        (* Procedure ResetVault *)
        /\ vault_ = defaultInitValue
        (* Procedure Withdraw *)
        /\ vault_W = defaultInitValue
        /\ master_ = defaultInitValue
        /\ to = defaultInitValue
        /\ delta_W = defaultInitValue
        (* Procedure QueuePendingWithraw *)
        /\ vault = defaultInitValue
        /\ master = defaultInitValue
        /\ delta = defaultInitValue
        /\ stack = << >>
        /\ pc = "Start"

TransferLabel == /\ pc = "TransferLabel"
                 /\ Assert(from /= to_, 
                           "Failure of assertion at line 79, column 9.")
                 /\ IF (acc_balance[from] >= delta_) /\ (delta_ > 0)
                       THEN /\ pc' = "FundSrc"
                            /\ UNCHANGED << stack, from, to_, delta_ >>
                       ELSE /\ pc' = Head(stack).pc
                            /\ from' = Head(stack).from
                            /\ to_' = Head(stack).to_
                            /\ delta_' = Head(stack).delta_
                            /\ stack' = Tail(stack)
                 /\ UNCHANGED << steps, acc_balance, vault_masters, 
                                 vault_withdraw, vault_cancel_master, vault_, 
                                 vault_W, master_, to, delta_W, vault, master, 
                                 delta >>

FundSrc == /\ pc = "FundSrc"
           /\ acc_balance' = [acc_balance EXCEPT ![from] = acc_balance[from] - delta_]
           /\ pc' = "FundDst"
           /\ UNCHANGED << steps, vault_masters, vault_withdraw, 
                           vault_cancel_master, stack, from, to_, delta_, 
                           vault_, vault_W, master_, to, delta_W, vault, 
                           master, delta >>

FundDst == /\ pc = "FundDst"
           /\ acc_balance' = [acc_balance EXCEPT ![to_] = acc_balance[to_] + delta_]
           /\ pc' = Head(stack).pc
           /\ from' = Head(stack).from
           /\ to_' = Head(stack).to_
           /\ delta_' = Head(stack).delta_
           /\ stack' = Tail(stack)
           /\ UNCHANGED << steps, vault_masters, vault_withdraw, 
                           vault_cancel_master, vault_, vault_W, master_, to, 
                           delta_W, vault, master, delta >>

Transfer == TransferLabel \/ FundSrc \/ FundDst

ResetLabel == /\ pc = "ResetLabel"
              /\ vault_cancel_master' = [vault_cancel_master EXCEPT ![vault_] = {}]
              /\ vault_withdraw' = [vault_withdraw EXCEPT ![vault_] = [Master |-> {}, Amount |-> 0]]
              /\ pc' = Head(stack).pc
              /\ vault_' = Head(stack).vault_
              /\ stack' = Tail(stack)
              /\ UNCHANGED << steps, acc_balance, vault_masters, from, to_, 
                              delta_, vault_W, master_, to, delta_W, vault, 
                              master, delta >>

ResetVault == ResetLabel

WithdrawLabel == /\ pc = "WithdrawLabel"
                 /\ Assert(MultiSigComplete(vault_W, master_), 
                           "Failure of assertion at line 104, column 9.")
                 /\ /\ delta_' = delta_W
                    /\ from' = vault_W
                    /\ stack' = << [ procedure |->  "Transfer",
                                     pc        |->  "WithdrawResetLabel",
                                     from      |->  from,
                                     to_       |->  to_,
                                     delta_    |->  delta_ ] >>
                                 \o stack
                    /\ to_' = to
                 /\ pc' = "TransferLabel"
                 /\ UNCHANGED << steps, acc_balance, vault_masters, 
                                 vault_withdraw, vault_cancel_master, vault_, 
                                 vault_W, master_, to, delta_W, vault, master, 
                                 delta >>

WithdrawResetLabel == /\ pc = "WithdrawResetLabel"
                      /\ /\ stack' = << [ procedure |->  "ResetVault",
                                          pc        |->  Head(stack).pc,
                                          vault_    |->  vault_ ] >>
                                      \o Tail(stack)
                         /\ vault_' = vault_W
                      /\ pc' = "ResetLabel"
                      /\ UNCHANGED << steps, acc_balance, vault_masters, 
                                      vault_withdraw, vault_cancel_master, 
                                      from, to_, delta_, vault_W, master_, to, 
                                      delta_W, vault, master, delta >>

Withdraw == WithdrawLabel \/ WithdrawResetLabel

QueueLabel == /\ pc = "QueueLabel"
              /\ Assert(MultiSigBegin(vault, master), 
                        "Failure of assertion at line 115, column 9.")
              /\ Assert(delta > 0, 
                        "Failure of assertion at line 116, column 9.")
              /\ vault_withdraw' = [vault_withdraw EXCEPT ![vault] = [Master |-> {master}, Amount |-> delta]]
              /\ pc' = Head(stack).pc
              /\ vault' = Head(stack).vault
              /\ master' = Head(stack).master
              /\ delta' = Head(stack).delta
              /\ stack' = Tail(stack)
              /\ UNCHANGED << steps, acc_balance, vault_masters, 
                              vault_cancel_master, from, to_, delta_, vault_, 
                              vault_W, master_, to, delta_W >>

QueuePendingWithraw == QueueLabel

Start == /\ pc = "Start"
         /\ steps' = 0
         /\ pc' = "Work"
         /\ UNCHANGED << acc_balance, vault_masters, vault_withdraw, 
                         vault_cancel_master, stack, from, to_, delta_, vault_, 
                         vault_W, master_, to, delta_W, vault, master, delta >>

Work == /\ pc = "Work"
        /\ IF steps < TOTAL_STEPS
              THEN /\ steps' = steps + 1
                   /\ Assert(SameTotalCoins, 
                             "Failure of assertion at line 130, column 9.")
                   /\ \/ /\ \E s \in {m \in MASTERS: acc_balance[m] > 0}:
                              \E d \in (MASTERS \ {s}):
                                \E a \in 1..acc_balance[s]:
                                  /\ /\ delta_' = a
                                     /\ from' = s
                                     /\ stack' = << [ procedure |->  "Transfer",
                                                      pc        |->  "Work",
                                                      from      |->  from,
                                                      to_       |->  to_,
                                                      delta_    |->  delta_ ] >>
                                                  \o stack
                                     /\ to_' = d
                                  /\ pc' = "TransferLabel"
                         /\ UNCHANGED <<vault_cancel_master, vault_, vault_W, master_, to, delta_W, vault, master, delta>>
                      \/ /\ \E v \in Vaults:
                              IF ~IsPendingWithdraw(v)
                                 THEN /\ \E m \in PairSet(vault_masters[v]):
                                           \E c \in 0..acc_balance[v]:
                                             IF c > 0
                                                THEN /\ /\ delta' = c
                                                        /\ master' = m
                                                        /\ stack' = << [ procedure |->  "QueuePendingWithraw",
                                                                         pc        |->  "Work",
                                                                         vault     |->  vault,
                                                                         master    |->  master,
                                                                         delta     |->  delta ] >>
                                                                     \o stack
                                                        /\ vault' = v
                                                     /\ pc' = "QueueLabel"
                                                ELSE /\ pc' = "Work"
                                                     /\ UNCHANGED << stack, 
                                                                     vault, 
                                                                     master, 
                                                                     delta >>
                                      /\ UNCHANGED << vault_W, master_, to, 
                                                      delta_W >>
                                 ELSE /\ \E m \in (PairSet(vault_masters[v]) \ vault_withdraw[v].Master):
                                           /\ /\ delta_W' = vault_withdraw[v].Amount
                                              /\ master_' = m
                                              /\ stack' = << [ procedure |->  "Withdraw",
                                                               pc        |->  "Work",
                                                               vault_W   |->  vault_W,
                                                               master_   |->  master_,
                                                               to        |->  to,
                                                               delta_W   |->  delta_W ] >>
                                                           \o stack
                                              /\ to' = RandomElement(vault_withdraw[v].Master)
                                              /\ vault_W' = v
                                           /\ pc' = "WithdrawLabel"
                                      /\ UNCHANGED << vault, master, delta >>
                         /\ UNCHANGED <<vault_cancel_master, from, to_, delta_, vault_>>
                      \/ /\ \E v \in Vaults:
                              IF ~IsPendingWithdraw(v)
                                 THEN /\ TRUE
                                      /\ pc' = "Work"
                                      /\ UNCHANGED << vault_cancel_master, 
                                                      stack, vault_ >>
                                 ELSE /\ \E m \in (PairSet(vault_masters[v]) \ vault_cancel_master[v]):
                                           /\ vault_cancel_master' = [vault_cancel_master EXCEPT ![v] = vault_cancel_master[v] \union {m}]
                                           /\ IF CanCancelWithdraw(v)
                                                 THEN /\ /\ stack' = << [ procedure |->  "ResetVault",
                                                                          pc        |->  "Work",
                                                                          vault_    |->  vault_ ] >>
                                                                      \o stack
                                                         /\ vault_' = v
                                                      /\ pc' = "ResetLabel"
                                                 ELSE /\ pc' = "Work"
                                                      /\ UNCHANGED << stack, 
                                                                      vault_ >>
                         /\ UNCHANGED <<from, to_, delta_, vault_W, master_, to, delta_W, vault, master, delta>>
              ELSE /\ pc' = "Done"
                   /\ UNCHANGED << steps, vault_cancel_master, stack, from, 
                                   to_, delta_, vault_, vault_W, master_, to, 
                                   delta_W, vault, master, delta >>
        /\ UNCHANGED << acc_balance, vault_masters, vault_withdraw >>

(* Allow infinite stuttering to prevent deadlock on termination. *)
Terminating == pc = "Done" /\ UNCHANGED vars

Next == Transfer \/ ResetVault \/ Withdraw \/ QueuePendingWithraw \/ Start
           \/ Work
           \/ Terminating

Spec == Init /\ [][Next]_vars

Termination == <>(pc = "Done")

\* END TRANSLATION - the hash of the generated TLA code (remove to silence divergence warnings): TLA-d4e40f7c5c29fd0cef450d3e3718fbf3


=============================================================================
