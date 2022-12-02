! Fortran float I11 edit descriptor
program integer
implicit none
integer(kind = 4) :: i, stat
integer(kind = 8) :: field
character(len = 11), dimension(35) :: integers
integers = [&
!Standard
"          0",&
"         +0",&
"         -0",&
"          1",&
"         +1",&
"         -1",&
!Whitespace Padding
"1          ",&
"     1     ",&
"          1",&
!Zero padding
" 0000000001",&
!Space
"1         2",&
"1 2 3 4 5 6",&
!Integers
"          1",&
"         12",&
"        123",&
"       1234",&
"      12345",&
"     123456",&
"    1234567",&
"   12345678",&
"  123456789",&
" 1234567890",&
! Invalid digit
"          a",&
"         -a",&
"         +a",&
! Invalid sign
"         *1",&
! Sign
"          -",&
"          +",&
! Double sign
"         -+",&
"         +-",&
"         ++",&
"         --",&
! Floats
" 12345678.9",&
" 1.2345E+12",&
! Blank
"           "&
]
do i = 1, size(integers)
    read(integers(i), "(I11)", iostat=stat)field
    if (stat == 0) then
        print '(A1, A11, A4, I11)', "|", integers(i), "| = ", field
    else
        print '(A1, A11, A4, A11)', "|", integers(i), "| = ", "invalid"
    end if
end do
end program
