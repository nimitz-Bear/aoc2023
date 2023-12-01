
def main(lines: [str]):
    sums = []

    for line in lines:
        first = ""
        last = ""
        for c in line:
            
            if c.isdigit():
                last = c
            
                # set the first char if it's a digit (and never set it again)
                if first == "":
                    first = c
        print(first + last)
        sums.append(int(first+last))
        
        
    print(sum(sums))
                
def part2(lines: [str]):
    sums = []

    for line in lines:
        
        digits = {"one": "1", "two": "2", "three": "3", "four": "4", "five":"5", "six":"6", "seven":"7", "eight":"8", "nine":"9"}
        
        # see if any of the digits in word form exist, and if they do, add a number beside them
        for word in digits.keys():
            i = line.find(word)
            if (i >= 0):
                # kinda cursed
                line = line[:i] + word + digits[word] + word + line[i:]
        
        
            
        first = ""
        last = ""
        for c in line:
            if c.isdigit():
                last = c
            
                # set the first char if it's a digit (and never set it again)
                if first == "":
                    first = c
        print(first + last)
        print(line)
        sums.append(int(first+last))
        
        
    print(sum(sums))
    
    
if __name__ == '__main__':
    file1 = open('inputs/day1.txt', 'r')
    lines = file1.readlines()
    # main(lines)
    part2(lines)