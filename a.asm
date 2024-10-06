    .org $0600

    LDX #$00            
    LDA #$00            
loop:
    .org $0700
    INX                 
    STA $01             
    CPX #$09           
    BNE $loop            
    BRK                 
