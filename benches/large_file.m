% simple_large_mat.m
% Creates a large .mat file with random numbers

N = 15000;  % Increase this to make a larger file (e.g. 30000, 40000, etc.)
A = rand(N, N);  % Creates an N x N matrix of doubles (~8*N^2 bytes)
save('large.mat', 'A', '-v7');  % Save in large-file format

