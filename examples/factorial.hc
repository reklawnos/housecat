var s = 1
var x = 1
while x < 1000001 do
    s = (s * x) % 4231432143214
    x = x + 1
end
print(s)
