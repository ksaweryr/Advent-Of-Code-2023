print((lambda a:(sum((x:=[int(c)for c in w if c.isdigit()],x[0]*10+x[-1])[1]for w in a),sum((m:=dict(zip('one two three four five six seven eight nine'.split(),range(1,10)))|{str(i):i for i in range(1,10)})[(r:=i('re')).match('.*?([1-9]|one|two|three|four|five|six|seven|eight|nine)',w).groups(1)[0]]*10+m[r.match('.*([1-9]|one|two|three|four|five|six|seven|eight|nine)',w).groups(1)[0]]for w in a)))((i:=__import__)('sys').stdin.read().strip().split()))