compile day_num part_num:
    cc -O2 -DNDEBUG -march=native -flto ./day{{day_num}}_part{{part_num}}.c -o day{{day_num}}_part{{part_num}}

run day_num part_num: (compile day_num part_num)
    ./day{{day_num}}_part{{part_num}}

bench day_num part_num: (compile day_num part_num)
    hyperfine --warmup 10 './day{{day_num}}_part{{part_num}}'