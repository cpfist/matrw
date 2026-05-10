% Generate file `examples_v7.mat`:
% Generate binary MAT-file of version 7.0 containing 
% example data.

% Scalar values
a = 42;
b = pi;
c = 1 + 2i;
d = 'abc';

% Multidimensional matrix
A(:,:,1) = [1 2 3; 4 5 6; 7 8 9];
A(:,:,2) = [10 11 12; 13 14 15; 16 17 18];

% Struct
s.field1 = 42;
s.field2 = 1234;

save('example_v7.mat', '-v7');
clear;

% Generate file `examples_v73.mat`:
% Generate binary MAT-file of version 7.3 containing 
% no data.

save('example_v73.mat','-v7.3');

