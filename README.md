# **Counter Program**

This project is a Solana smart contract written using the Anchor framework. The program manages a simple counter stored on the blockchain and provides functionality to initialize, increment, and decrement the counter.

---

## **Features**

1. **Initialize:** Creates a new counter account and sets its initial value to 0.
2. **Increment:** Safely increments the counter value by 1.
3. **Decrement:** Safely decrements the counter value by 1.

---

## **Installation**

### **Prerequisites**
- Install [Rust](https://www.rust-lang.org/tools/install)
- Install [Anchor CLI](https://www.anchor-lang.com/docs/installation)
- Install Solana CLI from [Solana Docs](https://docs.solana.com/cli/install-solana-cli-tools)

### **Steps**
1. Clone the repository:
   ```bash
   git clone https://github.com/<your-username>/counter-program.git
   cd counter-program
   anchor build
   anchor deploy

