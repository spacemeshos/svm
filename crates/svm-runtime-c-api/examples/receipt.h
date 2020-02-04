#ifndef SVM_RECEIPT
#define SVM_RECEIPT

#include <stdint.h>

#include "svm.h"
#include "func_rets.h"

typedef struct {
  bool success;
  uint8_t count;
  svm_func_ret_t *returns;
  
  svm_byte_array new_state;
  char* error;
} svm_receipt_t;

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

    svm_func_ret_t* returns = NULL;

    if (count > 0) {
      returns = (svm_func_ret_t*)(malloc(sizeof(svm_func_ret_t) * count));
      if (returns == NULL) {
	exit(-1);
      }
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
	printf("\n\nReceipt has no returns.\n");
    }
  }
  else {
    // ...
  }
}

#endif
