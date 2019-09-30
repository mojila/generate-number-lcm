disp("Bab 3 nomor 1");

xs = 1;
x = [];
length = 100;

function generated_number = lcm(xs1)
    a = 3;
    c = 15;
    m = 2571;
    
    generated_number = rem((a * xs1 + c), m);
    return;
end

for i = 1:length
    if i == 1
        x = lcm(xs);
    else
        x = [x lcm(x(1,i-1))];
    end
end

disp("Hasil generate 100 angka tidak sama");
disp(x);
