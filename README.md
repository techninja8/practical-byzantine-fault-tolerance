# PBFT Consensus Skeleton (Practical Byzantine Fault Tolerance)

This project provides a **skeleton implementation** of the **Practical Byzantine Fault Tolerance (PBFT)** consensus protocol in Rust. It intended to serves as a structured foundation for understanding **fault-tolerant consensus mechanisms** used in blockchain and distributed systems. Right now, this implmentation is far from function, my goal is to eventually optimize PBFT with faster block finality and reduced message overhead.

## 📌 What is PBFT?
PBFT is a consensus algorithm designed for **fault-tolerant distributed networks**, allowing a set of nodes to agree on a decision **even if some nodes are faulty or malicious** (Byzantine failures).

### ✅ **How PBFT Works**
PBFT operates in **five phases** to ensure **consensus** and **fault tolerance**:
1. **Client sends a request** to the **primary node**.
2. **Pre-prepare:** The primary node orders the request and broadcasts it to replicas.
3. **Prepare:** Replicas verify and confirm the request.
4. **Commit:** Replicas finalize the request and reach consensus.
5. **Reply:** Nodes send confirmation back to the client.

**PBFT can tolerate up to ⌊(N-1)/3⌋ Byzantine nodes** in a network of `N` nodes.

---

## 📌 Project Structure
This skeleton provides a **modular Rust implementation** with function prototypes and pseudo-code for each PBFT phase.

pbft-skeleton/ 
    │── src/ 
    │ ├── main.rs # Entry point, initializes the PBFT node 
    │ ├── node.rs # Defines the Node struct and state transitions 
    │ ├── pbft.rs # PBFT message types and consensus phases 
    │ ├── network.rs # Skeleton for peer-to-peer message passing 
    │ ├── utils.rs # Utility functions (hashing, cryptography) 
│── Cargo.toml # Rust dependencies 
│── README.md # Project documentation


---

## 📌 Features
This repo contains a couple of files modularly developed to implement various aspects of the traditional PBFT

✅ **PBFT Message Structure:** Defines request, pre-prepare, prepare, commit, and reply phases.  
✅ **Node State Handling:** Nodes transition between **Primary, Replica, and Committing** states.  
✅ **Networking Skeleton:** Basic **message passing** between nodes (to be extended).  
✅ **Crypto Placeholder:** Uses **SHA-256 hashing** (signature validation can be added).  

---

## 📌 Setup & Running the Project
### **1️⃣ Clone the Repository**
This repo could serve as a great starting point for similar PBFT implementations, so feel free to clone it and update it to your requirements

```sh
git clone https://github.com/yourusername/pbft-skeleton.git
cd pbft-skeleton

---

## 📌 Next Steps & Contributions
This is a starter implementation, and additional features can be added:

🔹 Implement Digital Signatures (ECDSA, Ed25519)
🔹 Extend P2P Networking (libp2p, Tokio)
🔹 Simulate Multiple PBFT Nodes
🔹 Run Consensus with Real Transactions

---

## Contributions are welcome! 🚀
Feel free to fork, submit issues, or create pull requests.

---

##📌 License
This project is licensed under the MIT License.

