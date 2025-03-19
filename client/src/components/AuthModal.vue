<template>
  <div>
    <!-- Login modal -->
    <Login v-if="activeModal === 'login'" 
      v-model="showModal" 
      @switch-to-signup="switchToSignUp" 
      ref="loginRef" />
    
    <!-- SignUp modal -->
    <SignUp v-if="activeModal === 'signup'" 
      v-model="showModal" 
      @switch-to-login="switchToLogin" 
      ref="signupRef" />
  </div>
</template>

<script setup>
import { ref, watch } from 'vue';
import { nextTick } from 'vue';
import Login from '@/components/Login.vue';
import SignUp from '@/components/SignUp.vue';
import { Clerk } from "@clerk/clerk-js";
import { cache } from "@/utils/cache";

const clerkPubKey = import.meta.env.VITE_CLERK_PUBLISHABLE_KEY;
const clerk = new Clerk(clerkPubKey);

const showModal = ref(true);
const activeModal = ref('login');
const loginRef = ref(null);
const signupRef = ref(null);

const openLogin = () => {
  // Clear all cache before opening auth modal to ensure fresh state
  cache.clearAll();
  activeModal.value = 'login';
  showModal.value = true;
  
  nextTick(() => {
    const signInDiv = document.getElementById("sign-in");
    if (signInDiv) {
      clerk.mountSignIn(signInDiv);
    }
  });
};

const openSignUp = () => {
  // Clear all cache before opening auth modal to ensure fresh state
  cache.clearAll();
  activeModal.value = 'signup';
  showModal.value = true;

  nextTick(() => {
    const signUpDiv = document.getElementById("sign-up");
    if (signUpDiv) {
      clerk.mountSignUp(signUpDiv);
    }
  });
};

const closeModal = () => {
  showModal.value = false;
};

const switchToSignUp = () => {
  activeModal.value = 'signup';
};

const switchToLogin = () => {
  activeModal.value = 'login';
};

watch(activeModal, () => {
  nextTick(() => {
    if (activeModal.value === 'login') {
      const signInDiv = document.getElementById("sign-in");
      if (signInDiv) {
        clerk.mountSignIn(signInDiv);
      }
    } else {
      const signUpDiv = document.getElementById("sign-up");
      if (signUpDiv) {
        clerk.mountSignUp(signUpDiv);
      }
    }
  });
});

defineExpose({ openLogin, openSignUp, closeModal });
</script>