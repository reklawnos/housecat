let iter = fn(count) -> ret {
  ret = ${
    cur_value: 0
    next: fn(self) -> ret {
      if self.cur_value < count do
        ret = (true, self.cur_value)
      else
        ret = (false, nil)
      end
      self.cur_value: self.cur_value + 1
    }
  }
}

let enumerate_and_add = fn(iter, amount) -> ret {
  ret = ${
    cur_value: 0
    next: fn(self) -> ret {
      let has_more, let next_val = iter|next()
      if has_more do
        ret = (true, (self.cur_value + amount, next_val))
        self.cur_value: self.cur_value + 1
      else
        ret = (false, nil)
      end
    }
  }
}

for i in iter(5) do
  print(i)
  print("this happened!")
end


for (e, i) in enumerate_and_add(iter(3), 3) do
  print("with enumeration:")
  print(e)
  print(i)
end

let list = ${for i in iter(10) do @i: i + 1 end}
