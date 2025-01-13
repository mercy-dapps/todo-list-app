import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { TodoListApp } from "../target/types/todo_list_app";
import { assert } from "chai";

describe("todo-list-app", () => {
  // Configure the client to use the local cluster.

  const provider = anchor.AnchorProvider.env();
  anchor.setProvider(provider);

  const program = anchor.workspace.TodoListApp as Program<TodoListApp>;

  let todoId;

  it("initialize user!", async () => {

    const [userPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from('user'),
        provider.wallet.publicKey.toBuffer()
      ],
      program.programId
    )

    const tx = await program.methods
      .initializeUser()
      .accountsPartial({
        user: userPda,
        author: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("Your transaction signature", tx);

    const userAccount = await program.account.user.fetch(userPda);
    console.log(userAccount);

    assert.equal(userAccount.author.toBase58(), provider.wallet.publicKey.toBase58());
    assert.equal(userAccount.todoCount, 0);
    assert.equal(userAccount.lastTodo, 0);
  });

  it("create task!", async () => {

    const [userPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from('user'),
        provider.wallet.publicKey.toBuffer()
      ],
      program.programId
    );

    const state = await program.account.user.fetch(userPda)
      todoId = state.lastTodo;

    const [taskPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from('task'),
        provider.wallet.publicKey.toBuffer(),
      ],
      program.programId
    );

    const tx = await program.methods
      .addTask("complete my todolist testing and deploy")
      .accountsPartial({
        user: userPda,
        task: taskPda,
        author: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("Your transaction signature", tx);

    const taskAccount = await program.account.task.fetch(taskPda);
    console.log(taskAccount);

    assert.equal(taskAccount.author.toBase58(), provider.wallet.publicKey.toBase58());
    assert.equal(taskAccount.title, "complete my todolist testing and deploy");
    assert.equal(taskAccount.completed, false);
  });

  it("update task!", async () => {

    const [userPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from('user'),
        provider.wallet.publicKey.toBuffer()
      ],
      program.programId
    );

    const [taskPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from('task'),
        provider.wallet.publicKey.toBuffer()
      ],
      program.programId
    );

    const tx = await program.methods
      .updateTask(todoId)
      .accountsPartial({
        user: userPda,
        task: taskPda,
        author: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("Your transaction signature", tx);

    const taskAccount = await program.account.task.fetch(taskPda);
    console.log(taskAccount);

    assert.equal(taskAccount.author.toBase58(), provider.wallet.publicKey.toBase58());
    assert.equal(taskAccount.title, "complete my todolist testing and deploy");
    assert.equal(taskAccount.completed, true);
  });

  it("edit task!", async () => {

    const [userPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from('user'),
        provider.wallet.publicKey.toBuffer()
      ],
      program.programId
    );

    const [taskPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from('task'),
        provider.wallet.publicKey.toBuffer()
      ],
      program.programId
    );

    const tx = await program.methods
      .editTask(todoId, "continuing my todolist testing and deploy")
      .accountsPartial({
        user: userPda,
        task: taskPda,
        author: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("Your transaction signature", tx);

    const taskAccount = await program.account.task.fetch(taskPda);
    console.log(taskAccount);

    assert.equal(taskAccount.author.toBase58(), provider.wallet.publicKey.toBase58());
    assert.equal(taskAccount.title, "continuing my todolist testing and deploy");
    assert.equal(taskAccount.completed, true);

    const userState = await program.account.user.fetch(userPda);
    const taskState = await program.account.task.fetch(taskPda);

    console.log(userState);
    console.log(taskState);     
  });

  it("delete task!", async () => {

    const [userPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from('user'),
        provider.wallet.publicKey.toBuffer()
      ],
      program.programId
    );

    const [taskPda] = anchor.web3.PublicKey.findProgramAddressSync(
      [Buffer.from('task'),
        provider.wallet.publicKey.toBuffer()
      ],
      program.programId
    );

    const tx = await program.methods
      .deleteTask(todoId)
      .accountsPartial({
        user: userPda,
        task: taskPda,
        author: provider.wallet.publicKey,
        systemProgram: anchor.web3.SystemProgram.programId,
      })
      .rpc();

    console.log("Your transaction signature", tx);

    const taskAccount = await program.account.task.all();
    console.log("Your task", taskAccount);

    assert.equal(taskAccount.length, 0);

    const userState = await program.account.user.fetch(userPda);

    console.log(userState);
  });
});
