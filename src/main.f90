subroutine test(a, n)
    implicit none
    integer :: n
    real(kind=8) :: a(n)
    print *, "Result: ", a(1), a(2)
    a(1) = 10.
    a(2) = 20.
end
