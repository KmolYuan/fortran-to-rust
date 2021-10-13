subroutine test(a, n)
    implicit none
    integer :: n
    real(kind=8) :: a(n)
    print *, "Fortran:", a(1), a(2)
    a(1) = 10.
    a(2) = 20.
end

subroutine test_2d(a, row, col)
    implicit none
    integer :: row, col
    real(kind=8) :: a(row, col)
    print *, "2d Array from Fortran:"
    print *, "[", a(1, 1), a(1, 2)
    print *, a(2, 1), a(2, 2), "]"
end
