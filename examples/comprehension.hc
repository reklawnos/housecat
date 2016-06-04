let range = fn(count) -> ret {
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

let enumerate = fn(iter) -> ret {
  ret = ${
    cur_value: 0
    next: fn(self) -> ret {
      let has_more, let next_val = iter|next()
      if has_more do
        ret = (true, (self.cur_value, next_val))
        self.cur_value: self.cur_value + 1
      else
        ret = (false, nil)
      end
    }
  }
}

for i in range(5) do
  print(i)
  print("this happened!")
end


print("with enumeration:")
for (e, i) in enumerate(range(3)) do
  print(e)
  print(i)
end


print(${
  a: "test"
  b: "wow"
})

let list = ${for i in range(10) do @i: i + 1 end}
