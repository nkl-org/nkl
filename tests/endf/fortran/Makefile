COMPILER = gfortran
FLAGS := -O3 -fbackslash

.PHONY: float integer

all: float integer

float: float.f90
	$(COMPILER) $(FLAGS) float.f90 -o float

integer: integer.f90
	$(COMPILER) $(FLAGS) integer.f90 -o integer
