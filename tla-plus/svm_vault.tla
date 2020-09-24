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
    LET All == [D -> MASTERS \X MASTERS]
    IN
    {      
        f \in All:
            \A x \in DOMAIN f:
                \E y, z \in MASTERS:
                    f[x] = <<y, z>> /\ (Ord(x) > Ord(y)) /\ (Ord(y) > Ord(z))
    }


(*--algorithm Vault 
variables
    steps,
    src, 
    dst,
    amount,
    current_vault,
    acc_balance \in AccountBalance,
    vault_masters \in VaultMasters[{"M3"}],   
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
    
    NonNegBalanceInvriant == \A m \in MASTERS: acc_balance[m] >= 0
end define; 


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
                 d \in (MASTERS \ {src}),
                 a \in 1..acc_balance[s] do
                
                src := s;
                dst := d;
                amount := a;
                goto Fund;
                     
            end with;        
        or
            \* Withdraw
            with v \in Vaults do       
                if ~IsPendingWithdraw(v) then
                    with m \in PairSet(vault_masters[v]), 
                         c \in 0..acc_balance[v] 
                    do
                        if c > 0 then
                            assert MultiSigBegin(v, m);  
    
                            vault_withdraw[v] := [Master |-> {m}, Amount |-> c]
                        end if;
                    end with;     
                else                   
                    with m \in (PairSet(vault_masters[v]) \ vault_withdraw[v].Master)
                    do
                        assert MultiSigComplete(v, m); 
                        
                        amount := vault_withdraw[v].Amount;
                        src := v;
                        dst := RandomElement(vault_withdraw[v].Master);
                        vault_withdraw[v] := [Master |-> {}, Amount |-> 0];
                        
                        goto Fund;
                    end with;           
                end if;   
            end with;
        or
            \* Cancel a Pending Withdraw             
            with v \in Vaults do
                current_vault := v;
                
                \* Cancellation makes no sense if there's no pending withdraw
                if ~IsPendingWithdraw(v) then
                    goto Work;
                else   
                    with m \in (PairSet(vault_masters[v]) \ vault_cancel_master[v]) do
                        vault_cancel_master[v] := vault_cancel_master[v] \union {m};

                        if CanCancelWithdraw(v) then
                            goto ResetPendingCancellation;
                        else
                            goto Work;
                        end if;
                    end with;
                end if;
            end with;         
        end either;
        
        AfterWork:
            goto Work;  
            
        Fund:
            if (acc_balance[src] >= amount) /\ (amount > 0) then         
                FundSrc: 
                    acc_balance[src] := acc_balance[src] - amount;
                FundDst:
                    acc_balance[dst] := acc_balance[dst] + amount;   
          
                goto Work;
            else
                goto Work;
            end if;
 
        ResetPendingCancellation:
            vault_cancel_master[current_vault] := {};
            vault_withdraw[current_vault] := [Master |-> {}, Amount |-> 0];
            goto Work;

    
    end while;
end algorithm;*)
\* BEGIN TRANSLATION - the hash of the PCal code: PCal-46db657ede04d52f5bbb3ad68eee4e3d
CONSTANT defaultInitValue
VARIABLES steps, src, dst, amount, current_vault, acc_balance, vault_masters, 
          vault_withdraw, vault_cancel_master, pc

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

NonNegBalanceInvriant == \A m \in MASTERS: acc_balance[m] >= 0


vars == << steps, src, dst, amount, current_vault, acc_balance, vault_masters, 
           vault_withdraw, vault_cancel_master, pc >>

Init == (* Global variables *)
        /\ steps = defaultInitValue
        /\ src = defaultInitValue
        /\ dst = defaultInitValue
        /\ amount = defaultInitValue
        /\ current_vault = defaultInitValue
        /\ acc_balance \in AccountBalance
        /\ vault_masters \in VaultMasters[{"M3"}]
        /\ vault_withdraw = [v \in Vaults |-> [Master |-> {}, Amount |-> 0]]
        /\ vault_cancel_master = [v \in Vaults |-> {}]
        /\ pc = "Start"

Start == /\ pc = "Start"
         /\ steps' = 0
         /\ pc' = "Work"
         /\ UNCHANGED << src, dst, amount, current_vault, acc_balance, 
                         vault_masters, vault_withdraw, vault_cancel_master >>

Work == /\ pc = "Work"
        /\ IF steps < TOTAL_STEPS
              THEN /\ steps' = steps + 1
                   /\ Assert(SameTotalCoins, 
                             "Failure of assertion at line 89, column 9.")
                   /\ \/ /\ \E s \in {m \in MASTERS: acc_balance[m] > 0}:
                              \E d \in (MASTERS \ {src}):
                                \E a \in 1..acc_balance[s]:
                                  /\ src' = s
                                  /\ dst' = d
                                  /\ amount' = a
                                  /\ pc' = "Fund"
                         /\ UNCHANGED <<current_vault, vault_withdraw, vault_cancel_master>>
                      \/ /\ \E v \in Vaults:
                              IF ~IsPendingWithdraw(v)
                                 THEN /\ \E m \in PairSet(vault_masters[v]):
                                           \E c \in 0..acc_balance[v]:
                                             IF c > 0
                                                THEN /\ Assert(MultiSigBegin(v, m), 
                                                               "Failure of assertion at line 111, column 29.")
                                                     /\ vault_withdraw' = [vault_withdraw EXCEPT ![v] = [Master |-> {m}, Amount |-> c]]
                                                ELSE /\ TRUE
                                                     /\ UNCHANGED vault_withdraw
                                      /\ pc' = "AfterWork"
                                      /\ UNCHANGED << src, dst, amount >>
                                 ELSE /\ \E m \in (PairSet(vault_masters[v]) \ vault_withdraw[v].Master):
                                           /\ Assert(MultiSigComplete(v, m), 
                                                     "Failure of assertion at line 119, column 25.")
                                           /\ amount' = vault_withdraw[v].Amount
                                           /\ src' = v
                                           /\ dst' = RandomElement(vault_withdraw[v].Master)
                                           /\ vault_withdraw' = [vault_withdraw EXCEPT ![v] = [Master |-> {}, Amount |-> 0]]
                                           /\ pc' = "Fund"
                         /\ UNCHANGED <<current_vault, vault_cancel_master>>
                      \/ /\ \E v \in Vaults:
                              /\ current_vault' = v
                              /\ IF ~IsPendingWithdraw(v)
                                    THEN /\ pc' = "Work"
                                         /\ UNCHANGED vault_cancel_master
                                    ELSE /\ \E m \in (PairSet(vault_masters[v]) \ vault_cancel_master[v]):
                                              /\ vault_cancel_master' = [vault_cancel_master EXCEPT ![v] = vault_cancel_master[v] \union {m}]
                                              /\ IF CanCancelWithdraw(v)
                                                    THEN /\ pc' = "ResetPendingCancellation"
                                                    ELSE /\ pc' = "Work"
                         /\ UNCHANGED <<src, dst, amount, vault_withdraw>>
              ELSE /\ pc' = "Done"
                   /\ UNCHANGED << steps, src, dst, amount, current_vault, 
                                   vault_withdraw, vault_cancel_master >>
        /\ UNCHANGED << acc_balance, vault_masters >>

AfterWork == /\ pc = "AfterWork"
             /\ pc' = "Work"
             /\ UNCHANGED << steps, src, dst, amount, current_vault, 
                             acc_balance, vault_masters, vault_withdraw, 
                             vault_cancel_master >>

Fund == /\ pc = "Fund"
        /\ IF (acc_balance[src] >= amount) /\ (amount > 0)
              THEN /\ pc' = "FundSrc"
              ELSE /\ pc' = "Work"
        /\ UNCHANGED << steps, src, dst, amount, current_vault, acc_balance, 
                        vault_masters, vault_withdraw, vault_cancel_master >>

FundSrc == /\ pc = "FundSrc"
           /\ acc_balance' = [acc_balance EXCEPT ![src] = acc_balance[src] - amount]
           /\ pc' = "FundDst"
           /\ UNCHANGED << steps, src, dst, amount, current_vault, 
                           vault_masters, vault_withdraw, vault_cancel_master >>

FundDst == /\ pc = "FundDst"
           /\ acc_balance' = [acc_balance EXCEPT ![dst] = acc_balance[dst] + amount]
           /\ pc' = "Work"
           /\ UNCHANGED << steps, src, dst, amount, current_vault, 
                           vault_masters, vault_withdraw, vault_cancel_master >>

ResetPendingCancellation == /\ pc = "ResetPendingCancellation"
                            /\ vault_cancel_master' = [vault_cancel_master EXCEPT ![current_vault] = {}]
                            /\ vault_withdraw' = [vault_withdraw EXCEPT ![current_vault] = [Master |-> {}, Amount |-> 0]]
                            /\ pc' = "Work"
                            /\ UNCHANGED << steps, src, dst, amount, 
                                            current_vault, acc_balance, 
                                            vault_masters >>

(* Allow infinite stuttering to prevent deadlock on termination. *)
Terminating == pc = "Done" /\ UNCHANGED vars

Next == Start \/ Work \/ AfterWork \/ Fund \/ FundSrc \/ FundDst
           \/ ResetPendingCancellation
           \/ Terminating

Spec == Init /\ [][Next]_vars

Termination == <>(pc = "Done")

\* END TRANSLATION - the hash of the generated TLA code (remove to silence divergence warnings): TLA-7f9f9c55bd5354ca6c73fdd897b01bf9


=============================================================================
