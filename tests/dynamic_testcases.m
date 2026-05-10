% Generate testcases

%
% Numeric arrays
%
numeric_var_u8 = uint8([1,2;3,4]);
numeric_var_i8 = int8([-1,2;3,4]);
numeric_var_u16 = uint16([1,2;3,4]);
numeric_var_i16 = int16([-1,2;3,4]);
numeric_var_u32 = uint32([1,2;3,4]);
numeric_var_i32 = int32([-1,2;3,4]);
numeric_var_u64 = uint64([1,2;3,4]);
numeric_var_i64 = int64([-1,2;3,4]);
numeric_var_f32 = single([-1,2;3,4]);
numeric_var_f64 = double([-1,2;3,4]);
numeric_var_char = ['1','2';'3','4'];

%
% Struct arrays
%
struct_var = struct;

struct_arr(1,10).a = 1;
struct_arr(1,10).b = 2;

S = struct(...
    'id',num2cell(1:4),...
    'name',{'Alice','Bob','Charlie','Dana'},...
    'age',num2cell([25 34 29 42]),...
    'score',num2cell([88.5 72.3 91.7 65.2]),...
    'passed',num2cell([88.5 72.3 91.7 65.2] >= 70));

% Specific 
%
if exist('OCTAVE_VERSION', 'builtin')
% Octave specific testcases

save('dynamic_octave_testcases.mat', '-v7');
else
% MATLAB specific testcases

% function handle
string_var = "test string";

save('dynamic_matlab_testcases.mat', '-v7');
end

