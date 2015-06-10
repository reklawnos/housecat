var iter = fn(count) -> ret {
    ret = ${
        cur_value: 0
        count: count
        next: fn(self) -> ret {
            if self.cur_value < self.count do
                ret = self.cur_value
            else
                ret = nil
            end
            self.cur_value: self.cur_value + 1
        }
    }
}

if false do
    print("yay")
elif false do
    print("boo")
elif false do
    print("huh?")
else
    print("confusion")
end


for i in iter(10) do
    print(i)
    print("this happened!")
end

for i in iter(5) do
    print(i)
    print("this happened!")
end

for i in iter(3) do
    print(i)
    print("this happened!")
end

print("got to here")

var list = ${for i in iter(10) do @i: i + 1 end}

for i in iter(10) do
    print(list[i])
end
