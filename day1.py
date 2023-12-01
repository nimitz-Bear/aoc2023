
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
        print("Before: " + line)
        # see if any of the digits in word form exist, and if they do, add a number beside them
        for word in digits.keys():
            i = line.find(word)
            if (i >= 0):
                line = line.replace(word, word + digits[word] + word)
        # line.replace("one", "one1one").replace("two", "two2two").replace("three", "three3three").replace("four", "four4four").replace("five", "five5five").replace("six", "six6six").replace("seven", "seven7seven").replace("eight", "eight8eight").replace("nine", "nine9nine").filter { isdigit() }
        
            
        first = ""
        last = ""
        for c in line:
            if c.isdigit():
                last = c
            
                # set the first char if it's a digit (and never set it again)
                if first == "":
                    first = c
        print(first + last)
        print("After: " + line)
     
        sums.append(int(first+last))
        
        
    print(sum(sums))
    
    
if __name__ == '__main__':
    file1 = open('inputs/day1.txt', 'r')
    lines = file1.readlines()
    # main(lines)
    part2(lines)