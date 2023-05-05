.inesprg 1   ; one (1) bank of program code
.ineschr 1   ; one (1) bank of picture data
.inesmap 0   ; we use mapper 0
.inesmir 1   ; Mirror setting always 1.

.bank 0   ; bank 0.
	.org $8000  ; goto location $8000.
  lda #%00001000 
	sta $2000
	lda #%00011110
	sta $2001

.bank 1     ; change to bank 1
	.org $FFFA  ; start at $FFFA

	.dw 0        ; location of NMI Interrupt
	.dw Start    ; code to run at reset, we give address of Start label that will go in bank 0
	
	.dw 0        ; location of VBlank Interrupt


.bank 2        ; change to bank 2
	.org $0000    ; start at $0000

	.incbin "our.bkg"  ; INClude BINary file that will contain our background pic
	; data.
	.incbin "our.spr"  ; INClude BINary file that will contain our sprite pic data
