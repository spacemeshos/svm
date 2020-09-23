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
    
    Range(f) == {f[x] : x \in DOMAIN f}
    
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
    
    VaultPendingAmount == VAULT.Pending.Amount
    
    AccountByMaster(M) == CHOOSE ACC \in Range(ACCOUNTS): ACC.Master = M
          
    NonNegeBalanceInvariant == (\A I \in 1..ACCOUNT_COUNT: ACCOUNTS[1].Balance >= 0)
                                    /\
                                (VAULT.Balance >= 0)
           
    TotalCoinsInvariant == SeqSum([I \in 1..ACCOUNT_COUNT |-> ACCOUNTS[I].Balance]) 
                           + VAULT.Balance 
                         = TOTAL_COINS                        
end define;
    

macro FundVault(FROM, AMOUNT)
begin
    assert FROM.Balance >= AMOUNT;
    
    FROM.Balance := FROM.Balance - AMOUNT;
    VAULT.Balance := VAULT.Balance + AMOUNT;
end macro;

macro FundAccount(ACC, AMOUNT) 
begin
    ACC.Balance := ACC.Balance + AMOUNT
end macro;

macro WithdrawBegin(ACC, AMOUNT) 
begin
    assert ~VaultIsPending;
    assert VAULT.Balance >= AMOUNT;
    
    VAULT.Pending := [Master |-> ACC.Master, Amount |-> AMOUNT]
end macro;

macro WithdrawApprove(ACC)
begin
    assert VaultIsPending;
    assert VAULT.Balance >= VaultPendingAmount;
    
    if (ACC.Master \in VAULT.Masters) /\ (ACC.Master /= VaultPendingMaster) then
        VAULT.Balance := VAULT.Balance - VaultPendingAmount;
        
        \* FundAccount(AccountByMaster(VaultPendingMaster), VaultPendingAmount);
        \* VAULT.Pending := [Master |-> {}, Amount |-> {}];
    end if;  
end macro;

begin
    while STEP < TOTAL_STEPS do     
        \* we always increment `STEP`
        STEP := STEP + 1;
        
        either 
            \* Withdraw (Begin / Approve)
            
            if ~VaultIsPending then
                \* Withdraw begin
                
                with I \in 1..ACCOUNT_COUNT,
                    AMOUNT \in 0..VAULT.Balance
                do
                    WithdrawBegin(ACCOUNTS[I], AMOUNT)
                end with;
                
            else
                \* Withdraw approve
                 
                 with M \in VAULT.Masters do
                    WithdrawApprove(AccountByMaster(M))
                 end with;
            end if; 
        or
            \* Fund Vault
            with I \in 1..ACCOUNT_COUNT,
                AMOUNT \in 0..ACCOUNTS[I].Balance
            do
                FundVault(ACCOUNTS[I], AMOUNT)
            end with; 
        or
            \* Next layer    
            LAYER := LAYER + 1  
        end either;
    end while;

end algorithm; *)

\* BEGIN TRANSLATION - the hash of the PCal code: PCal-f4e07fd92465c41f12c48923279fff80
VARIABLES STEP, LAYER, ACCOUNTS, VAULT, pc

(* define statement *)
Pick(S) == CHOOSE s \in S: TRUE

Range(f) == {f[x] : x \in DOMAIN f}

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

VaultPendingAmount == VAULT.Pending.Amount

AccountByMaster(M) == CHOOSE ACC \in Range(ACCOUNTS): ACC.Master = M

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
               THEN /\ STEP' = STEP + 1
                    /\ \/ /\ IF ~VaultIsPending
                                THEN /\ \E I \in 1..ACCOUNT_COUNT:
                                          \E AMOUNT \in 0..VAULT.Balance:
                                            /\ Assert(~VaultIsPending, 
                                                      "Failure of assertion at line 92, column 5 of macro called at line 125, column 21.")
                                            /\ Assert(VAULT.Balance >= AMOUNT, 
                                                      "Failure of assertion at line 93, column 5 of macro called at line 125, column 21.")
                                            /\ VAULT' = [VAULT EXCEPT !.Pending = [Master |-> (ACCOUNTS[I]).Master, Amount |-> AMOUNT]]
                                ELSE /\ \E M \in VAULT.Masters:
                                          /\ Assert(VaultIsPending, 
                                                    "Failure of assertion at line 100, column 5 of macro called at line 132, column 21.")
                                          /\ Assert(VAULT.Balance >= VaultPendingAmount, 
                                                    "Failure of assertion at line 101, column 5 of macro called at line 132, column 21.")
                                          /\ IF ((AccountByMaster(M)).Master \in VAULT.Masters) /\ ((AccountByMaster(M)).Master /= VaultPendingMaster)
                                                THEN /\ VAULT' = [VAULT EXCEPT !.Balance = VAULT.Balance - VaultPendingAmount]
                                                ELSE /\ TRUE
                                                     /\ VAULT' = VAULT
                          /\ UNCHANGED <<LAYER, ACCOUNTS>>
                       \/ /\ \E I \in 1..ACCOUNT_COUNT:
                               \E AMOUNT \in 0..ACCOUNTS[I].Balance:
                                 /\ Assert((ACCOUNTS[I]).Balance >= AMOUNT, 
                                           "Failure of assertion at line 79, column 5 of macro called at line 140, column 17.")
                                 /\ ACCOUNTS' = [ACCOUNTS EXCEPT ![I].Balance = (ACCOUNTS[I]).Balance - AMOUNT]
                                 /\ VAULT' = [VAULT EXCEPT !.Balance = VAULT.Balance + AMOUNT]
                          /\ LAYER' = LAYER
                       \/ /\ LAYER' = LAYER + 1
                          /\ UNCHANGED <<ACCOUNTS, VAULT>>
                    /\ pc' = "Lbl_1"
               ELSE /\ pc' = "Done"
                    /\ UNCHANGED << STEP, LAYER, ACCOUNTS, VAULT >>

(* Allow infinite stuttering to prevent deadlock on termination. *)
Terminating == pc = "Done" /\ UNCHANGED vars

Next == Lbl_1
           \/ Terminating

Spec == Init /\ [][Next]_vars

Termination == <>(pc = "Done")

\* END TRANSLATION - the hash of the generated TLA code (remove to silence divergence warnings): TLA-22f189e4a73912b5b86cf7fabd1ad1ca


=============================================================================
\* Modification History
\* Last modified Wed Sep 23 22:17:14 IDT 2020 by yaronwittenstein
\* Created Wed Sep 23 10:52:51 IDT 2020 by yaronwittenstein
