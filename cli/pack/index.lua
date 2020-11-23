#!/usr/bin/lua

local args = arg

local toAdd = {}

local output = "out.tar.gz"

for i,arg in pairs(args) do
	if i > 0 then
		if i ~= #args then
			table.insert(toAdd, arg)
		else
			output = arg
		end
	end
end

os.execute(
	"tar -czvf "
	..output
	..(function()
		local toReturn = ""
		for _,path in pairs(toAdd) do
			toReturn = toReturn.." \""..path.."\""
		end
		return toReturn
	end)())

