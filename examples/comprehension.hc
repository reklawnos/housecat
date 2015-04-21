var iter = fn(count) -> ret {
    ret = ${
        cur_value: 0
        count: count
        next: fn(self) -> ret {
            if self.cur_value < self.count
                ret = self.cur_value
            else
                ret = nil
            end
            self.cur_value: self.cur_value + 1
        }
    }
}


for i in iter(10)
    print(i)
    print("this happened!")
end

for i in iter(5)
    print(i)
    print("this happened!")
end

for i in iter(3)
    print(i)
    print("this happened!")
end

print("got to here")

var list = ${for i in iter(10) [i]: i + 1 end}

for i in iter(10)
    print(list[i])
end
