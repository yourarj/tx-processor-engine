## Description of cases covered

The following cases have been tested and covered under tests.

    1. case_01
       - deposit tx of value X
       - available and total should increase by amount X
    2. case_02
       - recreate state from case_01
       - withdraw tx of value X
       - available and total should decrease by amount X
    3. case_03
       - recreate state from case_02
       - mark tx Z of value X under dispute
       - attempt should fail with insufficient balance
    4. case_04
       - recreate state from case_01
       - mark tx Z of value X under dispute
       - tx Z should be marked under dispute    
    5. case_05
       - recreate state from case_04
       - mark tx Z as resolved
       - should succeed
       - held balances should get 
    6. case_06
       - recreate state from case_04
       - mark tx Z as chargebacked
       - should succeed
    7. case_05
       - recreate state from case_05
       - mark tx Z as chargebacked
       - should succeed
    8. case_06
    9. case_07
    10. case_08