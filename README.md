# CAPSTONE - PrismPapers: The Transparent Research Publishing and Reviewing Platform. [DeSci]

## Deployed on DevNet

- **Program ID:** [2nvhRn83KBxkkAfLH64meTq8cYB5aRLnZVbsxZdgfPTv](https://solscan.io/account/2nvhRn83KBxkkAfLH64meTq8cYB5aRLnZVbsxZdgfPTv?cluster=devnet)
- **Live Frontend:** [https://prism-papers-dapp.vercel.app/](https://prism-papers-dapp.vercel.app/)

## [PrismPapers - Architectural Diagram](https://app.eraser.io/workspace/srtYuk9xciXjH6IUKwJs)

## Project Overview: [PrismPapers - Teaser](https://ai.invideo.io/watch/eWg44dhBM4n)

PrismPapers is a blockchain-based platform designed to shake things up in academic publishing. Traditional academic publishing takes forever, costs too much, and is often opaque.

PrismPapers is my way of fixing this mess. With blockchain, we can make the whole process speedy, transparent, and rewarding for everyone involved. Plus, I’m a tech geek who loves using blockchain to solve real-world problems.

### Project Setup Pre-requisites:

- [rustup 1.27.1 stable](https://www.rust-lang.org/tools/install)
  ```bash
  rustup default stable
  ```
- [solana-cli 2.1.5 (client : Agave)](https://docs.solana.com/cli/install-solana-cli-tools)
  ```bash
  agave-install init 2.1.5
  ```
- [anchor-cli 0.31.1](https://www.anchor-lang.com/docs/installation)
  ```bash
  avm use 0.31.1
  ```

<br>

## Run Anchor Program and Tests Locally:

- **Anchor Program Files:** `anchor/programs/prismpapersdapp/src`
- **Test File:** `anchor/tests/prismpapersdapp.spec.ts`

<!-- end list -->

1.  **Clone the repository:**

    ```bash
    git clone https://github.com/AhindraD/prism-papers-dapp.git
    ```

2.  **Change the directory to the project folder:**

    ```bash
    cd prism-papers-dapp
    ```

3.  **Install dependencies:**

    ```bash
    pnpm install
    ```

4.  **Change directory to the anchor folder:**

    ```bash
    cd anchor
    ```

5.  **Build the program:**

    ```bash
    anchor build
    ```

6.  **Test the program:**
    You can run the tests using Anchor (which utilizes Vitest via the configured scripts):

    ```bash
    # Run using the script defined in package.json
    pnpm run anchor-test
    ```

    _Note: Ensure you are running a local validator or configured for Devnet in `Anchor.toml`._
