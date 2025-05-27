import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TodoListApp } from "../target/types/todo_list_app";
import { assert } from "chai";
import { randomBytes } from "crypto";
import {
  Keypair,
  LAMPORTS_PER_SOL,
  SystemProgram,
  Transaction,
} from "@solana/web3.js";

describe("todo-list-app", () => {
  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const connection = provider.connection;

  const program = anchor.workspace.TodoListApp as Program<TodoListApp>;

  const todo_id = randomBytes(8)[0];
  const todo2_id = randomBytes(8)[0];

  const author = Keypair.generate();

  const user = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("user"), author.publicKey.toBuffer()],
    program.programId
  )[0];

  const todo = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("todo"), author.publicKey.toBuffer(), Buffer.from([todo_id])],
    program.programId
  )[0];

  const todo_2 = anchor.web3.PublicKey.findProgramAddressSync(
    [Buffer.from("todo"), author.publicKey.toBuffer(), Buffer.from([todo2_id])],
    program.programId
  )[0];

  const accounts = {
    user,
    todo,
    author: author.publicKey,
  };

  const confirm = async (signature: string): Promise<string> => {
    const block = await connection.getLatestBlockhash();

    await connection.confirmTransaction({
      signature,
      ...block,
    });

    return signature;
  };

  const log = async (signature: string): Promise<string> => {
    console.log(
      `Your transaction signature: https://explorer.solana.com/transaction/${signature}?cluster=custom&customUrl=${connection.rpcEndpoint}`
    );

    return signature;
  };

  it("airdrop", async () => {
    let tx = new Transaction();
    tx.instructions = [
      SystemProgram.transfer({
        fromPubkey: provider.publicKey,
        toPubkey: author.publicKey,
        lamports: 1 * LAMPORTS_PER_SOL,
      }),
    ];

    await provider.sendAndConfirm(tx).then(log);
  });

  it("initialize user!", async () => {
    const tx = await program.methods
      .initializeUser()
      .accounts({ ...accounts })
      .signers([author])
      .rpc()
      .then(confirm)
      .then(log);

    console.log("Your transaction signature", tx);

    const userAccount = await program.account.user.fetch(user);
    console.log(userAccount);

    assert.equal(userAccount.author.toBase58(), author.publicKey.toBase58());
    assert.equal(userAccount.todoCount, 0);
  });

  it("create todo!", async () => {
    await program.methods
      .addTodo(todo_id, "complete my todolist testing and deploy")
      .accounts({ ...accounts })
      .signers([author])
      .rpc()
      .then(confirm)
      .then(log);

    await program.methods
      .addTodo(todo2_id, "start working on the ui")
      .accounts({
        todo: todo_2,
        author: author.publicKey,
      })
      .signers([author])
      .rpc()
      .then(confirm)
      .then(log);

    const todoAccount = await program.account.todo.fetch(todo);
    console.log(todoAccount);

    assert.equal(todoAccount.author.toBase58(), author.publicKey.toBase58());
    assert.equal(todoAccount.title, "complete my todolist testing and deploy");
    assert.equal(todoAccount.completed, false);
  });

  it("update todo!", async () => {
    const tx = await program.methods
      .updateTodo(todo_id)
      .accounts({ ...accounts })
      .signers([author])
      .rpc();

    console.log("Your transaction signature", tx);

    const todoAccount = await program.account.todo.fetch(todo);
    console.log(todoAccount);

    assert.equal(todoAccount.author.toBase58(), author.publicKey.toBase58());
    assert.equal(todoAccount.title, "complete my todolist testing and deploy");
    assert.equal(todoAccount.completed, true);
  });

  it("edit todo!", async () => {
    const tx = await program.methods
      .editTodo(todo_id, "continuing my todolist testing and deploy")
      .accounts({ ...accounts })
      .signers([author])
      .rpc();

    console.log("Your transaction signature", tx);

    const todoAccount = await program.account.todo.fetch(todo);
    console.log(todoAccount);

    assert.equal(todoAccount.author.toBase58(), author.publicKey.toBase58());
    assert.equal(
      todoAccount.title,
      "continuing my todolist testing and deploy"
    );
    assert.equal(todoAccount.completed, true);

    const userState = await program.account.user.fetch(user);
    const todoState = await program.account.todo.fetch(todo);

    console.log(userState);
    console.log(todoState);
  });

  it("delete todo!", async () => {
    const tx = await program.methods
      .deleteTodo(todo_id)
      .accounts({ ...accounts })
      .signers([author])
      .rpc();

    console.log("Your transaction signature", tx);

    const todoAccount = await program.account.todo.all();
    console.log("Your todo", todoAccount);

    assert.equal(todoAccount.length, 1);

    const userState = await program.account.user.fetch(user);

    console.log(userState);
  });
});
