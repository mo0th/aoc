questions = None

with open('input.txt', 'r') as f:
    questions = f.read().splitlines()


def operation(op, num1, num2):
    if op == '+':
        return int(num1) + int(num2)
    elif op == '*':
        return int(num1) * int(num2)


def evaluate(expression):
    expr_len = len(expression)
    i = 0
    result = 0
    acc_type = None
    op = '+'
    acc = ''

    while i < expr_len:
        ch = expression[i]

        if ch == ')':
            result = operation(op, result, evaluate(acc))
            acc = ''

        elif ch == ' ':
            if acc.isdigit():
                result = operation(op, result, acc)
            acc = ''

        elif ch in ['+', '*']:
            op = ch

        elif ch == '(':
            acc = '('
            depth = 1
            i += 1
            while depth > 0:
                ch = expression[i]
                acc += ch
                if ch == ')':
                    depth -= 1
                if ch == '(':
                    depth += 1
                i += 1
            result = operation(op, result, evaluate(acc[1:-1]))
            acc = ''

        elif ch.isdigit():
            acc += ch

        i += 1

    if acc:
        result = operation(op, result, acc)

    return result


def part1():
    answers = [evaluate(q) for q in questions[:]]

    print('final', sum(answers))


part1()
