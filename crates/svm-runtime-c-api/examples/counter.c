#include "svm.h"

#include <stdlib.h>
#include <stdio.h>
#include <string.h>
#include <assert.h>

typedef struct {
  uint32_t counter;
} host_t;

typedef struct {
  uint8_t* bytes;
  long length;
} wasm_file_t;

typedef struct {
  svm_byte_array app_addr;
  svm_byte_array init_state;
} spawned_app_t;

typedef struct {
  uint8_t slice_count;
  svm_byte_array* slices;
} svm_func_buf_t;

typedef struct {
  svm_value_type type;
  uint8_t* bytes; 
} svm_func_arg_t;

typedef struct {
  uint8_t arg_count;
  svm_func_arg_t* args;
} svm_func_args_t;

typedef struct {
  uint8_t type;
  uint32_t i32_value;
  uint64_t i64_value;
} svm_func_ret_t;

typedef struct {
  bool success;
  uint8_t count;
  svm_func_ret_t *returns;
  svm_byte_array new_state;
  char* error;
} svm_receipt_t;

uint32_t func_buf_length(svm_func_buf_t func_buf) {
  uint32_t acc = 0;

  acc += 1; // `#func-buf #slices` consumes 1 byte

  for (uint8_t i = 0; i < func_buf.slice_count; i++) {
    svm_byte_array slice = func_buf.slices[i];
    acc += slice.length;
  }

  return acc;
}

uint32_t func_args_length(svm_func_args_t func_args) {
  uint32_t acc = 0;

  acc += 1; // `#func args` consumes 1 byte

  for (uint8_t i = 0; i < func_args.arg_count; i++) {
    svm_func_arg_t arg = func_args.args[i];
    acc += 1; // `arg type` consumes 1 byte

    svm_value_type arg_type = arg.type;
    if (arg_type == SVM_I32) {
      acc += 4; // `arg` takes 4 bytes
    }
    else if (arg_type == SVM_I64) {
      acc += 8; // `arg` takes 8 bytes
    }
    else {
      // ilegal argument type
      exit(-1);
    }
  }

  return acc;
}

wasm_file_t read_wasm_file(const char *file_name) {
  wasm_file_t wasm_file;

  FILE *file = fopen(file_name, "r");
  fseek(file, 0, SEEK_END);
  wasm_file.length = ftell(file);

  wasm_file.bytes = malloc(wasm_file.length);
  fseek(file, 0, SEEK_SET);
  fread(wasm_file.bytes, 1, wasm_file.length, file);
  fclose(file);

  return wasm_file;
}

svm_byte_array deploy_template_bytes() {
  wasm_file_t file = read_wasm_file("wasm/counter.wasm");

  uint64_t length =
    4  +  // proto version
    1  +  // name length 
    strlen("Example")  +  // length("Example")
    2  +  // `#admins`    (we'll set it to `0`)
    2  +  // `#deps`      (we'll set it to `0`)
    2  +  // `page_count` (we'll set it to `0`)
    8  +  //  code length (Big-Endian)
    (uint64_t)file.length; // code

  uint8_t* bytes = (uint8_t*)(malloc(length));

  uint32_t cursor = 0;

  // set `proto=0`
  bytes[0] = 0;
  bytes[1] = 0;
  bytes[2] = 0;
  bytes[3] = 0;
  cursor += 4;

  // name length takes 1 bytes
  bytes[cursor] = strlen("Example");
  cursor += 1;

  const char* name = "Example";
  memcpy(&bytes[cursor], name, strlen("Example"));
  cursor += strlen("Example");

  // `#admins` takes 2 bytes
  bytes[cursor + 0] = 0;
  bytes[cursor + 1] = 0;
  cursor += 2;

  // `#deps` takes 2 bytes
  bytes[cursor + 0] = 0;
  bytes[cursor + 1] = 0;
  cursor += 2;

  // `#page_count` takes 2 bytes
  bytes[cursor + 0] = 0;
  bytes[cursor + 1] = 0;
  cursor += 2;

  // set code-length (Big-Endian)
  uint8_t* code_length = (uint8_t*)&file.length;

  for (int i = 0; i < 8; i++) {
    // we assume `code_length` is in Little-Endian order,
    // so we reverse it since a raw template format expects it in Big-Endian order.
    bytes[cursor + i] = code_length[7 - i];
  }
  cursor += 8;

  // copy template code
  memcpy(&bytes[cursor], file.bytes, file.length);

  svm_byte_array template = {
    .bytes = bytes,
    .length = length
  };

  return template;
}

svm_byte_array spawn_app_bytes(svm_byte_array template_addr) {
  uint64_t length =
    4  +  // proto version
    template_addr.length +  // length(`template_addr)
    1 +  // ctor #slices
    1;   // ctor func #args

  uint8_t* bytes = (uint8_t*)(malloc(length));

  uint32_t cursor = 0;

  // set `proto=0`
  bytes[0] = 0;
  bytes[1] = 0;
  bytes[2] = 0;
  bytes[3] = 0;
  cursor += 4;

  // copy `template` address
  memcpy(&bytes[cursor], template_addr.bytes, template_addr.length);
  cursor += template_addr.length;

  // `ctor buf #slices` take 1 byte 
  bytes[cursor] = 0; // no `ctor func buf`
  cursor += 1;

  // `ctor #args` take 1 byte 
  bytes[cursor] = 0; // no `ctor func args`
  cursor += 1;

  svm_byte_array app = {
    .bytes = bytes,
    .length = length
  };

  return app;
}

svm_byte_array host_ctx_empty_bytes() {
  uint32_t length =
    4  +  // proto version
    2;   // #fields

  uint8_t* bytes = (uint8_t*)(malloc(length));
  
  uint32_t cursor = 0;

  // set `proto=0`
  bytes[0] = 0;
  bytes[1] = 0;
  bytes[2] = 0;
  bytes[3] = 0;
  cursor += 4;

  // set `#fields=0`
  bytes[cursor + 0] = 0;
  bytes[cursor + 1] = 0;
  cursor += 2;

  svm_byte_array host_ctx = {
    .bytes = bytes,
    .length = length
  };

  return host_ctx;
}

svm_byte_array exec_app_bytes(
    svm_byte_array app_addr,
    svm_byte_array func_name,
    svm_func_buf_t func_buf,
    svm_func_args_t func_args
) {
  uint32_t length =
    4   +  // proto version
    20  +  // app address
    1   +  // function name length
    func_name.length +  
    func_buf_length(func_buf) + 
    func_args_length(func_args);

  uint8_t* bytes = (uint8_t*)(malloc(length));  
  uint32_t cursor = 0;

  // set `proto=0`
  bytes[0] = 0;
  bytes[1] = 0;
  bytes[2] = 0;
  bytes[3] = 0;
  cursor += 4;

  // set `app` address
  memcpy(&bytes[cursor], app_addr.bytes, app_addr.length);
  cursor += app_addr.length;
  
  // `name length` consumes 1 byte
  assert(func_name.length <= 0xFF);
  bytes[cursor] = (uint8_t)func_name.length; 
  cursor += 1;

  // set `name length`
  memcpy(&bytes[cursor], func_name.bytes, func_name.length);
  cursor += func_name.length;

  // function buf

  //// `func buf #slices` conumes 1 byte
  bytes[cursor] = func_buf.slice_count; 
  cursor += 1;

  for (uint8_t i = 0; i < func_buf.slice_count; i++) {
    svm_byte_array slice = func_buf.slices[i];

    assert(slice.length <= 0xFFFF);

    //// we assume `slice_len` is laid out in Little-Endian in memory.
    uint16_t slice_len = (uint16_t)slice.length; 

    //// slice length consumes 2 bytes (Big-Endian)
    bytes[cursor + 0] = (uint8_t)((slice_len >> 8) & 0xFF);
    bytes[cursor + 1] = (uint8_t)((slice_len >> 0) & 0xFF); 
    cursor += 2;

    //// copy slice to `buf`
    memcpy(&bytes[cursor], slice.bytes, slice.length);
    cursor += slice.length;
  }

  // function args

  //// `func #args` consumes 1 byte
  bytes[cursor] = func_args.arg_count; 
  cursor += 1;

  //// copy `func args` to `buf`
  for (uint8_t i = 0; i < func_args.arg_count; i++) {
    svm_func_arg_t arg = func_args.args[i];

    svm_value_type arg_type = arg.type;

    //// arg type consumes 1 byte
    bytes[cursor] = (uint8_t)arg_type;
    cursor += 1;

    if (arg_type == SVM_I32) {
      for (uint8_t off = 0; off < 4; off++) {
	bytes[cursor + off] = *(arg.bytes + 3 - off); 
      }

      cursor += 4; //// arg value takes 4 bytes
    }
    else if (arg_type == SVM_I64) {
      for (uint8_t off = 0; off < 8; off++) {
	bytes[cursor + off] = *(arg.bytes + 7 - off); 
      }
      
      cursor += 8; //// arg value takes 8 bytes
    }
    else {
      //// ilegal argument type
      exit(-1);
    }
  }

  assert(cursor == length);

  svm_byte_array tx = {
    .length = length,
    .bytes = bytes
  };

  return tx;
}

host_t* host_new(uint32_t counter_initial) {
  host_t* host = (host_t*)malloc(sizeof(host_t));
  host->counter = counter_initial;

  return host;
}

void host_inc_counter(void *ctx, uint32_t value) {
  host_t *host = (host_t*)(svm_instance_context_host_get(ctx));
  host->counter = host->counter + value;
}

uint32_t host_get_counter(void *ctx) {
  host_t *host = (host_t*)(svm_instance_context_host_get(ctx));
  return host->counter;
}

void inc_counter_import_build(void* imports) { 
  svm_byte_array module_name = {
    .bytes = (const uint8_t *)"env",
    .length = strlen("env")
  };

  svm_byte_array import_name = {
    .bytes = (const uint8_t *)"inc_counter",
    .length = strlen("inc_counter")
  };

  svm_value_type* types = (svm_value_type*)malloc(sizeof(svm_value_type) * 1);
  types[0] = SVM_I32;

  svm_value_type_array params = {
    .types = types,
    .length = 1
  };

  svm_value_type_array returns = {
    .types = NULL,
    .length = 0
  };

  void* func = (void*)host_inc_counter;

  svm_result_t res = svm_import_func_build(imports, module_name, import_name, func, params, returns);
  assert(res == SVM_SUCCESS);
}

void get_counter_import_build(void* imports) {
  svm_byte_array module_name = {
    .bytes = (const uint8_t *)"env",
    .length = strlen("env")
  };

  svm_byte_array import_name = {
    .bytes = (const uint8_t *)"get_counter",
    .length = strlen("get_counter")
  };

  svm_value_type_array params = {
    .types = NULL,
    .length = 0
  };

  svm_value_type* types = (svm_value_type*)malloc(sizeof(svm_value_type) * 1);
  types[0] = SVM_I32;

  svm_value_type_array returns = {
    .types = types,
    .length = 1
  };

  void* func = (void*)host_get_counter;

  svm_result_t res = svm_import_func_build(imports, module_name, import_name, func, params, returns);
  assert(res == SVM_SUCCESS);
}

void* imports_build() {
  uint32_t length = 2;
  void* imports = NULL;

  svm_result_t res = svm_imports_alloc(&imports, length);
  assert(res == SVM_SUCCESS);

  inc_counter_import_build(imports);
  get_counter_import_build(imports);

  return imports;
}

void* runtime_create(void* imports) {
  // create a new kv-store
  void *kv = NULL;
  svm_memory_kv_create(&kv);

  uint32_t balance = 12;
  host_t* host = host_new(balance);

  void *runtime = NULL;
  svm_result_t res = svm_memory_runtime_create(&runtime, kv, host, imports); 
  assert(res == SVM_SUCCESS); 
  return runtime;
}

void* alloc_byte_address(uint8_t byte) {
  uint8_t *addr = (uint8_t*)malloc(20);
  memset(addr, byte, 20);
  return (void*)addr;
}

void* alloc_author_addr() {
  return alloc_byte_address(0xAA);
}  

void* alloc_creator_addr() {
  return alloc_byte_address(0xBB);
}  

void* alloc_sender_addr() {
  return alloc_byte_address(0xCC);
}  

void* alloc_empty_state() {
  uint8_t *state = (uint8_t*)malloc(32);
  memset(state, 0, 32);
  return (void*)state;
}

svm_receipt_t decode_receipt(svm_byte_array encoded_receipt) {
  uint32_t cursor = 0;

  const uint8_t* bytes = encoded_receipt.bytes;

  for (; cursor < 4; cursor++) {
    assert(bytes[cursor] == 0);
    cursor += 1;
  }

  uint8_t success = bytes[cursor];
  cursor += 1;

  if (success) {
    assert(cursor == 5);

    // `new state`
    uint8_t* new_state_bytes = (uint8_t*)malloc(sizeof(uint8_t) * 32);

    if (new_state_bytes == NULL) {
      exit(-1);
    }

    memcpy(new_state_bytes, bytes + cursor, 32);
    cursor += 32;

    svm_byte_array new_state;
    new_state.bytes = new_state_bytes;
    new_state.length = 32;

    // `#returns`
    uint8_t count = bytes[cursor];
    cursor += 1;

    svm_func_ret_t* returns = (svm_func_ret_t*)(malloc(sizeof(svm_func_ret_t) * count));
    if (returns == NULL) {
      exit(-1);
    }

    for(uint8_t i = 0; i < count; i++) {
      svm_func_ret_t *ret = returns + i;

      uint8_t ret_type = bytes[cursor];
      cursor += 1;

      ret->type = ret_type;

      if (ret_type == SVM_I32) {
        uint32_t i32_value = 0;

    	for(uint8_t off = 0; off < 4; off++) {
    	  uint8_t byte = bytes[cursor];
    	  cursor += 1;

    	  i32_value += (byte << (3 - off));
    	}

    	ret->i32_value = i32_value;
      }
      else if (ret_type == SVM_I64) {
        uint64_t i64_value = 0;

    	for(uint8_t off = 0; off <8; off++) {
    	  uint8_t byte = bytes[cursor];
    	  cursor += 1;

    	  i64_value += (byte << (7 - off));
    	}

    	ret->i64_value = i64_value;
      }
      else {
    	exit(-1);
      }
    }

    svm_receipt_t receipt = {
      .success = true,
      .count = count,
      .returns = returns, 
      .new_state = new_state,
      .error = NULL
    };

    return receipt;
  }
  else {
    svm_receipt_t receipt = {
      .success = false
    };

    return receipt;
  }
}

svm_byte_array simulate_deploy_template(void* runtime, svm_byte_array bytes, void* author) {
  svm_byte_array host_ctx = host_ctx_empty_bytes();

  svm_byte_array template_addr; 
  svm_result_t res = svm_deploy_template(&template_addr, runtime, author, host_ctx, bytes); 

  assert(res == SVM_SUCCESS); 

  printf("Deployed AppTemplate successfully...\n");
  for(int i = 0; i < template_addr.length; i++) {
    printf("%d ", template_addr.bytes[i]);
  }
  printf("\n\n");

  return template_addr;
}

spawned_app_t simulate_spawn_app(void* runtime, svm_byte_array bytes, void* creator) {
  svm_byte_array host_ctx = host_ctx_empty_bytes();

  svm_byte_array app_addr; 
  svm_byte_array init_state; 
  svm_result_t res = svm_spawn_app(&app_addr, &init_state, runtime, creator, host_ctx, bytes);
  assert(res == SVM_SUCCESS); 

  printf("Spawned App successfully...\n");
  printf("App Account Address:\n");

  for (int i = 0; i < app_addr.length; i++) {
    printf("%d ", app_addr.bytes[i]);
  }

  printf("\n\n");
  printf("App initial state:\n");
  for (int i = 0; i < init_state.length; i++) {
    printf("%d ", init_state.bytes[i]);
  }
  printf("\n\n");

  spawned_app_t spawned = {
    .app_addr = app_addr,
    .init_state = init_state
  };
  return spawned;
}

void print_receipt(svm_receipt_t receipt) {
  if (receipt.success == true) {
    svm_byte_array new_state = receipt.new_state;

    printf("New app state:\n"); 

    for (uint8_t i = 0; i < new_state.length; i++) {
	printf("%02X ", new_state.bytes[i]);
    }

    if (receipt.count > 0) {
	printf("\n\nReceipt returns:\n");

	for (uint8_t i = 0; i < receipt.count; i++) {
	    svm_func_ret_t* ret = &receipt.returns[i];

	    if (i > 0) {
		printf(", ");
	    }

	    if (ret->type == SVM_I32) {
		printf("I32(%d)", ret->i32_value);
	    }
	    else if (ret->type == SVM_I64) {
		printf("I64(%llu)", ret->i64_value);
	    }
	    else {
		exit(-1);
	    }
        }

	printf("\n");
    }
    else {
	printf("\n\nReceipt has no returns:\n");
    }
  }
  else {
    // ...
  }
}

svm_byte_array simulate_get_balance(void* runtime, svm_byte_array app_addr, void* state, void* sender) {
  svm_byte_array func_name = { .bytes = (const uint8_t*)"get", .length = strlen("get") };
  svm_func_buf_t func_buf = { .slice_count = 0, .slices = NULL };
  svm_func_args_t func_args = { .arg_count = 0, .args = NULL };

  svm_byte_array bytes = exec_app_bytes(app_addr, func_name, func_buf, func_args);

  void *app_tx = NULL;
  svm_result_t res = svm_parse_exec_app(&app_tx, runtime, sender, bytes);
  assert(res == SVM_SUCCESS);

  svm_byte_array encoded_receipt;
  svm_byte_array host_ctx = host_ctx_empty_bytes();

  res = svm_exec_app(&encoded_receipt, runtime, app_tx, state, host_ctx);
  assert(res == SVM_SUCCESS);

  return encoded_receipt;
}

svm_byte_array simulate_inc_balance(void* runtime, svm_byte_array app_addr, void* state, void* sender, uint32_t inc_by) {
  svm_byte_array func_name = { .bytes = (const uint8_t*)"inc", .length = strlen("inc") };
  svm_func_buf_t func_buf = { .slice_count = 0, .slices = NULL };

  uint8_t arg_bytes[4];
  for (uint8_t i = 0; i < 4; i++) {
    arg_bytes[i] = (inc_by >> (3 - i) & 0xFF);
  }

  svm_func_arg_t arg;
  arg.type = (svm_value_type)SVM_I32;
  arg.bytes = (uint8_t*)&arg_bytes[0];

  svm_func_args_t func_args = { .arg_count = 1, .args = &arg };
  svm_byte_array bytes = exec_app_bytes(app_addr, func_name, func_buf, func_args);

  void *app_tx = NULL;
  svm_result_t res = svm_parse_exec_app(&app_tx, runtime, sender, bytes);
  assert(res == SVM_SUCCESS);

  svm_byte_array encoded_receipt;
  svm_byte_array host_ctx = host_ctx_empty_bytes();

  res = svm_exec_app(&encoded_receipt, runtime, app_tx, state, host_ctx);
  assert(res == SVM_SUCCESS);

  return encoded_receipt;
}
  
int main() {
  svm_byte_array bytes;

  void* imports = imports_build();
  void* runtime = runtime_create(imports);

  // 1) Deploy Template
  void* author = alloc_author_addr();
  bytes = deploy_template_bytes(); 
  svm_byte_array template_addr = simulate_deploy_template(runtime, bytes, author);

  // 2) Spawn App 
  void* creator = alloc_creator_addr();
  bytes = spawn_app_bytes(template_addr);
  spawned_app_t spawned = simulate_spawn_app(runtime, bytes, creator);
  svm_byte_array app_addr = spawned.app_addr;
  void* init_state = (void*)spawned.init_state.bytes;

  // 3) Exec App
  //// a) First we want to assert that the counter has been initialized as expected (see `create_import_object` above)  
  void* sender = alloc_sender_addr();
  svm_byte_array enc_receipt = simulate_get_balance(runtime, app_addr, init_state, sender);
  svm_receipt_t receipt = decode_receipt(enc_receipt);
  print_receipt(receipt);

  //// b) Increment the counter 
  void* new_state = (void*)receipt.new_state.bytes;
  uint32_t inc_by = 7;
  simulate_inc_balance(runtime, app_addr, new_state, sender, inc_by); 

  /* uint8_t *arg = int32_arg_new(7); */

  /*new_state_bytes,  length = exec_app_byte
  /*     sender_addr, */
  /*     app_addr, */
  /*     "inc", */
  /*     strlen("inc"), */
  /*     1,     // `args_count = 1` */
  /*     arg,   // `args_buf = [1, 0, 0, 0, 7]` */
  /* assert(res == SVM_SUCCESS); */
  /* free(bytes); */

  /* res = svm_exec_app(&receipt, runtime, app_tx, (void*)new_state); */
  /* assert(res == SVM_SUCCESS); */
  /* assert(svm_receipt_status(receipt) == true); */

  /* svm_receipt_results(&results, receipt, &results_len); */
  /* assert(results_len == 0); */

  /* length = exec_app_bytes( */
  /*     &bytes, */
  /*     app_addr, */
  /*     "get", */
  /*     strlen("get"), */
  /*     0,    // `args_count = 0` */
  /*     NULL, // `args_buf = NULL` */
  /*     0);   // `args_buf_len = 0` */

  /* res = svm_parse_exec_app(&app_tx, runtime, bytes, length); */
  /* assert(res == SVM_SUCCESS); */
  /* free(bytes); */
  /* assert(res == SVM_SUCCESS); */

  /* svm_receipt_results(&results, receipt, &results_len); */
  /* assert(results_len == 1); */

  // destroy...
  svm_runtime_destroy(runtime);
  svm_imports_destroy(imports);
}
