"""
http://adventofcode.com/2017/day/16
"""

function danceMove(programs::String, move::SubString{String})
    regex = r"(s|x|p)([0-9a-z]+)/?([0-9a-z]+)?"
    moveType,pos1,pos2 = match(regex, move).captures
    if moveType == "s"
        output = string(programs[end-parse(Int,pos1)+1:end], programs[1:end-parse(Int,pos1)])
    elseif moveType == "x"
        i,j = sort([parse(Int, pos1)+1, parse(Int, pos2)+1])
        output = string(programs[1:i-1], programs[j], programs[i+1:j-1], programs[i], programs[j+1:end])
    elseif moveType == "p"
        i,j = sort([i for (i,letter) in enumerate(programs) if string(letter) in [pos1,pos2]])
        output = string(programs[1:i-1], programs[j], programs[i+1:j-1], programs[i], programs[j+1:end])
    end
    return output
end

function makeDance(programs::String, moveList::Array{SubString{String},1})
    output = programs[1:end]
    for i in 1:length(moveList)
        move = moveList[i]
        output = danceMove(output, move)
    end
    return output
end


file = f = open("day16_input.txt")
s = readstring(f)
moves = split(s, ",")

programs = "abcdefghijklmnop"
programs_0 = "abcdefghijklmnop"

for i in 1:1000000000%42
    if programs == programs_0
        println(i)
    end
    programs = makeDance(programs, moves)
end

println(programs)