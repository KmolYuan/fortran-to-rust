module ten_mod
    implicit none
    public ten

contains

    function ten(input)
        implicit none
        integer :: input, ten

        print *, "Hello from the Fortran function!"

        ten = input * 10
    end

end
