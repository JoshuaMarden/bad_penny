
### Design Choices & Design Philosophy

1. Readable
2. Commenting is an indicator of poor readability
3. Maintainable > Robust > Reusable > Optimal
4. Avoid repetition
5. Try to produce crashes not bugs
6. Data Oriented Design (No OOP)
7. Pure functions
8. Encapsulate only at a high level, not at the micro level
9. Bundle globals into sensible structures
10. Constrain scope of variables
11. Long functions where they enhances readability

12. Break the right rules, at the right times




## Alternate philosophies / Comparison
___

### Summary:

1. **Procedural vs. Imperative**:
   - **Imperative** is giving step-by-step instructions.
   - **Procedural** organises those instructions into reusable blocks (functions).

2. **Procedural vs. Functional**:
   - **Procedural** uses functions, but they can modify variables and have side effects.
   - **Functional** functions are "pure" and don't modify anything outside their scope.

3. **Object-Oriented vs. Imperative**:
   - **Imperative** is a direct list of steps to perform.
   - **Object-Oriented** organises code into objects that have properties (data) and behaviours (methods).

4. **Object-Oriented vs. Functional**:
   - **Object-Oriented** focuses on objects and managing state.
   - **Functional** avoids state and side effects, using pure functions that just return values.

### 1. **Procedural vs. Imperative**

- **Imperative programming**: This is like giving step-by-step instructions on how to do something. You tell the computer **how** to do tasks, line by line, changing the program’s state as you go.

- **Procedural programming**: This is a type of imperative programming, but it focuses on organizing the code into **procedures** (also called **functions**). Instead of writing one big set of instructions, you split the instructions into reusable blocks (functions).

#### Example:
In Python:

**Imperative example** (step-by-step, without functions):
```python
x = 5
y = 10
sum = x + y
print(sum)
```
Here, you're directly telling the computer to do the steps in sequence. This is imperative because it focuses on how to do something.

**Procedural example** (using functions):
```python
def add_numbers(a, b):
    return a + b

result = add_numbers(5, 10)
print(result)
```
Now, we’re organizing the steps into a function called `add_numbers`. You can reuse this function, which is the hallmark of procedural programming.

**Contrast**:
- **Imperative**: Gives direct, step-by-step instructions.
- **Procedural**: Still gives step-by-step instructions but organizes them into reusable functions (procedures).

---

### 2. **Procedural vs. Functional**

- **Procedural programming**: As mentioned, it organizes the code into functions that may modify variables, have side effects (like printing or changing a global variable), and follow step-by-step instructions.

- **Functional programming**: Focuses on **what to do** rather than how to do it. It avoids changing state or mutating variables. Functions are "pure" (they don't change the outside world) and return values based only on their inputs.

#### Example:
In Python:

**Procedural example** (functions with side effects):
```python
def add_numbers(a, b):
    global total
    total = a + b  # Modifies the external state
    print(total)  # Side effect (printing)
    
add_numbers(5, 10)
```
This procedural function modifies a global variable and prints out the result, which are side effects.

**Functional example** (no side effects, pure function):
```python
def add_numbers(a, b):
    return a + b  # No side effects, just returns a result

result = add_numbers(5, 10)
print(result)
```
Here, the function does not modify anything outside itself. It simply takes inputs and returns a result.

**Contrast**:
- **Procedural**: Functions can have side effects, modify variables, and rely on global states.
- **Functional**: Functions are pure, no side effects, no modifying global state—just input/output.

---

### 3. **Object-Oriented vs. Imperative**

- **Imperative programming**: As discussed, it’s step-by-step instructions for the computer to follow, focusing on how things are done.

- **Object-Oriented programming (OOP)**: It’s all about organizing your code around **objects** (which are things in the real world or abstractions). Objects have properties (variables) and behaviors (methods/functions). It focuses on **who does what** (objects), not just how to do things.

#### Example:
In Python:

**Imperative example** (step-by-step):
```python
x = 5
y = 10
sum = x + y
print(sum)
```
Here, the program is directly doing the task, without thinking in terms of objects or behaviors.

**Object-Oriented example**:
```python
class Calculator:
    def __init__(self, a, b):
        self.a = a
        self.b = b

    def add(self):
        return self.a + self.b

calc = Calculator(5, 10)  # Creating an object of class Calculator
result = calc.add()       # Calling the add method
print(result)
```
Here, the logic is encapsulated in an **object** (`calc`), and we interact with the object’s behavior (its `add` method).

**Contrast**:
- **Imperative**: Direct step-by-step instructions without objects or classes.
- **Object-Oriented**: Organizes code around objects and their behaviors. The focus is on the **who** (objects) that do the task.

---

### 4. **Object-Oriented vs. Functional**

- **Object-Oriented programming (OOP)**: Organizes code into objects. Each object holds data (variables) and methods (functions) that act on that data. It often focuses on managing **state** (i.e., what is stored in an object).

- **Functional programming**: Emphasizes writing pure functions, avoiding state, and side effects. The focus is on what to return based on inputs rather than storing or managing data in objects.

#### Example:
In Python:

**Object-Oriented example**:
```python
class Counter:
    def __init__(self):
        self.count = 0
    
    def increment(self):
        self.count += 1
        return self.count

counter = Counter()
print(counter.increment())  # Prints 1
print(counter.increment())  # Prints 2
```
Here, the `Counter` object manages its internal state (`count`). Every time you call `increment()`, it changes the object's state.

**Functional example**:
```python
def increment(count):
    return count + 1  # Pure function, no state change

print(increment(0))  # Prints 1
print(increment(1))  # Prints 2
```
This function doesn’t rely on any object or state; it just takes an input and returns a new value based on that input.

**Contrast**:
- **Object-Oriented**: Organizes around objects and often changes their internal state.
- **Functional**: Avoids state and side effects, focusing only on pure functions.

---

### Summary:

1. **Procedural vs. Imperative**:
   - **Imperative** is giving step-by-step instructions.
   - **Procedural** organizes those instructions into reusable blocks (functions).

2. **Procedural vs. Functional**:
   - **Procedural** uses functions, but they can modify variables and have side effects.
   - **Functional** functions are "pure" and don't modify anything outside their scope.

3. **Object-Oriented vs. Imperative**:
   - **Imperative** is a direct list of steps to perform.
   - **Object-Oriented** organizes code into objects that have properties (data) and behaviors (methods).

4. **Object-Oriented vs. Functional**:
   - **Object-Oriented** focuses on objects and managing state.
   - **Functional** avoids state and side effects, using pure functions that just return values.

