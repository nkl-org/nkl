! Fortran float F11.0 edit descriptor
program float
implicit none
integer, parameter:: dp=kind(0.d0)
integer(kind = 4) :: i, stat
real(dp) :: field
character(len = 11), dimension(111) :: floats
floats = [&
!Standard
"        0.0",&
"       +0.0",&
"       -0.0",&
"        1.0",&
"       +1.0",&
"       -1.0",&
!Whitespace Padding
" 1.0       ",&
"    1.0    ",&
"        1.0",&
!Zero padding
" 1.00000000",&
" 0001.00000",&
" 00000001.0",&
!Decimal separator
"          .",&
!Sign
"          -",&
"          +",&
!Sign + decimal separator
"         -.",&
"         +.",&
!Null exponent
"        1E0",&
"       1E+0",&
"       1E-0",&
"  1E0000000",&
" 1E+0000000",&
" 1E-0000000",&
!Exponent part only
"         E1",&
"        E12",&
"       E123",&
"        E+1",&
"       E+12",&
"      E+123",&
"        E-1",&
"       E-12",&
"      E-123",&
!Decimal separator + exponent part
"        .E1",&
!Sign + decimal separator + exponent part
"       +.E1",&
"       -.E1",&
!End with decimal separator
"         0.",&
"         1.",&
"        -1.",&
!Space
"1    .    2",&
"1 . 2 e - 3",&
"1    e    2",&
"1.   e   -2",&
"- 1 . e + 2",&
"+ 1 . e - 2",&
"  - 1 . 2  ",&
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
! Floats
"         1.",&
"        1.2",&
"       1.23",&
"      1.234",&
"     1.2345",&
"    1.23456",&
"   1.234567",&
"  1.2345678",&
" 1.23456789",&
"1.234567890",&
! Scientific 1 digit
" 1.234567E1",&
" 1.23456E+1",&
" 1.23456E-1",&
! SciEntific 2 digits
" 1.23456E12",&
" 1.2345E+12",&
" 1.2345E-12",&
! SciEntific 3 digits
" 1.2345E123",&
" 1.234E+123",&
" 1.234E-123",&
! Scientific E-less 1 digit
" 1.234567+1",&
" 1.234567-1",&
! Scientific E-less 2 digits
" 1.23456+12",&
" 1.23456-12",&
! Scientific E-less 3 digits
" 1.2345+123",&
" 1.2345-123",&
! Scientific D format
" 1.2345d+12",&
" 1.2345D+12",&
! Special numbers
" NaN       ",&
" nan       ",&
" NAN       ",&
" inf       ",&
" INF       ",&
" infinity  ",&
" INFINITY  ",&
"-inf       ",&
"-infinity  ",&
"+inf       ",&
"+infinity  ",&
! Invalid digit
"          a",&
"         -a",&
"         +a",&
! Invalid sign
"         *1",&
! Double sign
"         +-",&
"         ++",&
"         --",&
! Exponent separator only
"          e",&
"          E",&
! Empty exponent part
"         1E",&
"         1+",&
"         1-",&
"        1E+",&
"        1E-",&
! Float exponent
"       e1.0",&
"      e+1.0",&
"      e-1.0",&
! Blank
"           "&
]
do i = 1, size(floats)
    read(floats(i), "(F11.0)", iostat=stat)field
    if (stat == 0) then
        print '(A1, A11, A4, ES17.9 E3)', "|", floats(i), "| = ", field
    else
        print '(A1, A11, A4, A17)', "|", floats(i), "| = ", "invalid"
    end if
end do
end program