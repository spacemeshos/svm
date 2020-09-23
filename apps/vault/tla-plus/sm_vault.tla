------------------------------ MODULE sm_vault ------------------------------
EXTENDS TLC, Integers, Sequences, FiniteSets

CONSTANTS MASTERS, ACCOUNT_COUNT, TOTAL_COINS, TOTAL_STEPS, NULL

ASSUME ACCOUNT_COUNT = 4
ASSUME Cardinality(MASTERS) = ACCOUNT_COUNT
ASSUME TOTAL_COINS \in 0..10
ASSUME TOTAL_STEPS \in 1..10

(* --algorithm sm_vault

variables   
    STEP = 0,
    
    LAYER = 0,
    
    ACCOUNTS =
        LET C == Pick(AllCoins)
            M1 == Pick(MASTERS)
            M2 == Pick(MASTERS \ {M1})
            M3 == Pick((MASTERS \ {M1}) \ {M2})
            M4 == Pick(((MASTERS \ {M1}) \ {M2}) \ {M3})
            M == <<M1, M2, M3, M4>>    
        IN
            LET ConsAcc(I) == [Master |-> M[I], Balance |-> C[I]]
            IN [I \in 1..ACCOUNT_COUNT |-> ConsAcc(I)],
            
         
    VAULT =  
        LET M1 == Pick(MASTERS)
            M2 == Pick(MASTERS \ {M1})
            M3 == Pick((MASTERS \ {M1}) \ {M2})
        IN
            [
                Masters |-> {M1, M2, M3},
                Balance |-> 0,
                Pending |-> [Master |-> {}, Amount |-> {}]
            ]

define    
    Pick(S) == CHOOSE s \in S: TRUE
    
    RECURSIVE SeqSum(_)
    SeqSum(S) ==
        IF S = <<>> THEN
            0
        ELSE
            Head(S) + SeqSum(Tail(S))
                                 
    AllCoins == 
        LET All == [1..ACCOUNT_COUNT -> 0..TOTAL_COINS]
        IN 
           {C \in All: SeqSum(C) = TOTAL_COINS}
           
    
    VaultIsPending == VAULT.Pending.Master /= {}
    
    VaultPendingMaster == VAULT.Pending.Master
    
    MasterAccount(M) == CHOOSE ACC \in ACCOUNTS: ACC.Master = M
          
    NonNegeBalanceInvariant == (\A I \in 1..ACCOUNT_COUNT: ACCOUNTS[1].Balance >= 0)
                                    /\
                                (VAULT.Balance >= 0)
           
    TotalCoinsInvariant == SeqSum([I \in 1..ACCOUNT_COUNT |-> ACCOUNTS[I].Balance]) 
                           + VAULT.Balance 
                         = TOTAL_COINS                        
end define;
    

macro Fund(FROM, AMOUNT)
begin
    assert FROM.Balance >= AMOUNT;
    
    FROM.Balance := FROM.Balance - AMOUNT;
    VAULT.Balance := VAULT.Balance + AMOUNT;
end macro;


macro WithdrawBegin(ACC, AMOUNT) 
begin
    assert ~VaultIsPending;
    assert ACC.Balance >= AMOUNT;
    
    VAULT.Pending := [Master |-> ACC.Master, Amount |-> AMOUNT]
end macro;

macro WithdrawApprove(ACC)
begin
    assert VaultIsPending
end macro;

begin
    while STEP < TOTAL_STEPS do
        either 
            \* Withdraw (Begin / Approve)
            
            if ~VaultIsPending then
                \* Withdraw begin
                
                with I \in 1..ACCOUNT_COUNT,
                    AMOUNT \in 0..ACCOUNTS[I].Balance
                do
                    WithdrawBegin(ACCOUNTS[I], AMOUNT)
                end with;
                
            else
                \* Withdraw approve
                 
                 with M \in (VAULT.Masters) do
                    skip;
                    \* WithdrawApprove(MasterAccount(M))
                 end with;
            end if; 
        or
            \* Fund Vault
            with I \in 1..ACCOUNT_COUNT,
                AMOUNT \in 0..ACCOUNTS[I].Balance
            do
                Fund(ACCOUNTS[I], AMOUNT)
            end with; 
        or
            \* Next layer    
            LAYER := LAYER + 1  
        end either;
        
        \* we always increment `STEP`
        STEP := STEP + 1;
    end while;

end algorithm; *)

\* BEGIN TRANSLATION - the hash of the PCal code: PCal-ce0a7dcf3e8d8cc4695ebfed35ea7ba9
VARIABLES STEP, LAYER, ACCOUNTS, VAULT, pc

(* define statement *)
Pick(S) == CHOOSE s \in S: TRUE

RECURSIVE SeqSum(_)
SeqSum(S) ==
    IF S = <<>> THEN
        0
    ELSE
        Head(S) + SeqSum(Tail(S))

AllCoins ==
    LET All == [1..ACCOUNT_COUNT -> 0..TOTAL_COINS]
    IN
       {C \in All: SeqSum(C) = TOTAL_COINS}


VaultIsPending == VAULT.Pending.Master /= {}

VaultPendingMaster == VAULT.Pending.Master

MasterAccount(M) == CHOOSE ACC \in ACCOUNTS: ACC.Master = M

NonNegeBalanceInvariant == (\A I \in 1..ACCOUNT_COUNT: ACCOUNTS[1].Balance >= 0)
                                /\
                            (VAULT.Balance >= 0)

TotalCoinsInvariant == SeqSum([I \in 1..ACCOUNT_COUNT |-> ACCOUNTS[I].Balance])
                       + VAULT.Balance
                     = TOTAL_COINS


vars == << STEP, LAYER, ACCOUNTS, VAULT, pc >>

Init == (* Global variables *)
        /\ STEP = 0
        /\ LAYER = 0
        /\ ACCOUNTS = (LET C == Pick(AllCoins)
                           M1 == Pick(MASTERS)
                           M2 == Pick(MASTERS \ {M1})
                           M3 == Pick((MASTERS \ {M1}) \ {M2})
                           M4 == Pick(((MASTERS \ {M1}) \ {M2}) \ {M3})
                           M == <<M1, M2, M3, M4>>
                       IN
                           LET ConsAcc(I) == [Master |-> M[I], Balance |-> C[I]]
                           IN [I \in 1..ACCOUNT_COUNT |-> ConsAcc(I)])
        /\ VAULT = (LET M1 == Pick(MASTERS)
                        M2 == Pick(MASTERS \ {M1})
                        M3 == Pick((MASTERS \ {M1}) \ {M2})
                    IN
                        [
                            Masters |-> {M1, M2, M3},
                            Balance |-> 0,
                            Pending |-> [Master |-> {}, Amount |-> {}]
                        ])
        /\ pc = "Lbl_1"

Lbl_1 == /\ pc = "Lbl_1"
         /\ IF STEP < TOTAL_STEPS
               THEN /\ \/ /\ IF ~VaultIsPending
                                THEN /\ \E I \in 1..ACCOUNT_COUNT:
                                          \E AMOUNT \in 0..ACCOUNTS[I].Balance:
                                            /\ Assert(~VaultIsPending, 
                                                      "Failure of assertion at line 84, column 5 of macro called at line 106, column 21.")
                                            /\ Assert((ACCOUNTS[I]).Balance >= AMOUNT, 
                                                      "Failure of assertion at line 85, column 5 of macro called at line 106, column 21.")
                                            /\ VAULT' = [VAULT EXCEPT !.Pending = [Master |-> (ACCOUNTS[I]).Master, Amount |-> AMOUNT]]
                                ELSE /\ \E M \in (VAULT.Masters):
                                          TRUE
                                     /\ VAULT' = VAULT
                          /\ UNCHANGED <<LAYER, ACCOUNTS>>
                       \/ /\ \E I \in 1..ACCOUNT_COUNT:
                               \E AMOUNT \in 0..ACCOUNTS[I].Balance:
                                 /\ Assert((ACCOUNTS[I]).Balance >= AMOUNT, 
                                           "Failure of assertion at line 75, column 5 of macro called at line 122, column 17.")
                                 /\ ACCOUNTS' = [ACCOUNTS EXCEPT ![I].Balance = (ACCOUNTS[I]).Balance - AMOUNT]
                                 /\ VAULT' = [VAULT EXCEPT !.Balance = VAULT.Balance + AMOUNT]
                          /\ LAYER' = LAYER
                       \/ /\ LAYER' = LAYER + 1
                          /\ UNCHANGED <<ACCOUNTS, VAULT>>
                    /\ STEP' = STEP + 1
                    /\ pc' = "Lbl_1"
               ELSE /\ pc' = "Done"
                    /\ UNCHANGED << STEP, LAYER, ACCOUNTS, VAULT >>

(* Allow infinite stuttering to prevent deadlock on termination. *)
Terminating == pc = "Done" /\ UNCHANGED vars

Next == Lbl_1
           \/ Terminating

Spec == Init /\ [][Next]_vars

Termination == <>(pc = "Done")

\* END TRANSLATION - the hash of the generated TLA code (remove to silence divergence warnings): TLA-0b7919d4c7bc59f2e5ad3df581893e43


=============================================================================
\* Modification History
\* Last modified Wed Sep 23 20:45:25 IDT 2020 by yaronwittenstein
\* Created Wed Sep 23 10:52:51 IDT 2020 by yaronwittenstein
