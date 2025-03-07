<!-- PrivacyPolicy.vue -->
<template>
  <div class="min-h-screen bg-[rgb(3,5,21)] relative">
    <!-- Gradient Overlay -->
    <div class="absolute top-0 right-0 w-full h-full animated-gradient"></div>
    <!-- Navigation -->
    <nav
      class="px-4 py-4 min-h-24 flex align-center border-b border-gray-700 bg-white/5 backdrop-filter backdrop-blur-3xl relative z-10">
      <div class="container mx-auto">
        <div class="flex justify-between items-center">
          <!-- Logo -->
          <div class="flex items-center space-x-2">
            <span class="text-white text-xl font-semibold"><a href="/">BetterNewTab_</a></span>
          </div>

          <!-- Navigation Links -->
          <div class="hidden md:flex space-x-8 border border-gray-300 px-6 py-4 rounded-full">
            <a href="/docs/getting-started" class="text-gray-300 hover:text-white transition-colors">Getting Started</a>
            <span class="text-gray-300 cursor-default">•</span>
            <a href="/docs/integrations" class="text-gray-300 hover:text-white transition-colors">Integrations</a>
            <span class="text-gray-300 cursor-default">•</span>
            <a href="/#pricing" class="text-gray-300 hover:text-white transition-colors">Pricing</a>
            <span class="text-gray-300 cursor-default">•</span>
            <a href="/docs/guides" class="text-gray-300 hover:text-white transition-colors">Guides</a>
            <span class="text-gray-300 cursor-default">•</span>
            <a href="/contact" class="text-gray-300 hover:text-white transition-colors">Contact</a>
          </div>

          <!-- Get Started and Sign In Buttons -->
          <div class="flex items-center space-x-4">
            <button @click="handleShowSignUp"
              class="px-4 py-4 rounded-l-full backdrop-filter backdrop-blur-lg text-white hover:bg-white/5 active:bg-white/10 transition-colors flex items-center space-x-2 shadow-inner shadow-white/20">
              <span class="text-lg">Get Started</span>
              <svg xmlns="http://www.w3.org/2000/svg" class="h-5 w-5" viewBox="0 0 24 24" fill="none"
                stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
                <path d="M5 12h14M12 5l7 7-7 7" />
              </svg>
            </button>
            <button @click="handleShowSignIn"
              class="px-4 py-4 rounded-r-full backdrop-filter backdrop-blur-lg text-white hover:bg-white/5 active:bg-white/10 transition-colors flex items-center space-x-2 shadow-inner shadow-white/20">
              <span class="text-lg">Sign In</span>
            </button>
          </div>
        </div>
      </div>
    </nav>

    <!-- Privacy Policy Section -->
    <div class="container mx-auto px-4 py-12">
      <div class="text-center max-w-4xl mx-auto mb-12">
        <h1 class="text-4xl md:text-6xl font-bold text-white mb-6 leading-tight">
          Privacy <span class="text-blue-400">Policy</span>
        </h1>
        <p class="text-xl text-gray-300">
          Last updated: {{ new Date().toLocaleDateString('en-US', { month: 'long', day: 'numeric', year: 'numeric' }) }}
        </p>
      </div>

      <div class="max-w-4xl mx-auto bg-white/5 backdrop-filter backdrop-blur-lg rounded-lg p-8 shadow-lg">
        <div class="prose prose-invert max-w-none">
          <p class="text-gray-300 mb-6">
            This Privacy Policy describes how BetterNewTab ("we", "us" or "our") collects, uses, and shares your personal information when you use our website and browser extension (the "Service").
          </p>

          <h2 class="text-2xl font-bold text-white mt-8 mb-4">Information We Collect</h2>
          
          <div class="text-gray-300">
            <p class="mb-4">We collect several types of information from and about users of our Service, including:</p>
            
            <ul class="list-disc pl-6 mb-6">
              <li class="mb-2"><strong>Personal Information:</strong> Email address and user name, collected through our authentication provider, Clerk.</li>
              <li class="mb-2"><strong>User Content:</strong> Links, URLs, and descriptions that you enter into the website.</li>
              <li class="mb-2"><strong>Search History:</strong> We store your last 500 search terms locally on your device. Search frequency is tracked and scored to provide better search suggestions. This data is stored locally on your device and is not transmitted to our servers.</li>
              <li class="mb-2"><strong>Keystrokes:</strong> We track keystrokes solely for the purpose of enabling keyboard shortcuts. These are not logged or stored.</li>
              <li class="mb-2"><strong>Search Terms:</strong> When you use our search suggestion feature, your search terms are sent to Brave to generate search suggestions using their API.</li>
            </ul>
          </div>

          <h2 class="text-2xl font-bold text-white mt-8 mb-4">How We Use Your Information</h2>
          
          <div class="text-gray-300">
            <p class="mb-4">We use the information we collect to:</p>
            
            <ul class="list-disc pl-6 mb-6">
              <li class="mb-2">Provide, maintain, and improve our Service</li>
              <li class="mb-2">Process and complete transactions, and send related information</li>
              <li class="mb-2">Send technical notices, updates, security alerts, and support messages</li>
              <li class="mb-2">Respond to your comments, questions, and requests</li>
              <li class="mb-2">Provide personalized content and search suggestions</li>
            </ul>
          </div>

          <h2 class="text-2xl font-bold text-white mt-8 mb-4">Data Storage and Transfer</h2>
          
          <p class="text-gray-300 mb-4">
            Your data is stored in Supabase, not on our servers. Data is transferred through our servers (located in Canada) to Supabase. Our operations are based in Canada, but we use various services such as Clerk, Supabase, and Brave that operate globally, primarily in the United States.
          </p>

          <h2 class="text-2xl font-bold text-white mt-8 mb-4">Error Tracking and Monitoring</h2>
          
          <p class="text-gray-300 mb-4">
            We use Sentry for error tracking and performance monitoring. Sentry may associate your email address and ID with crash details and telemetry data. This information is used solely for improving our Service and troubleshooting issues.
          </p>

          <h2 class="text-2xl font-bold text-white mt-8 mb-4">Analytics and Telemetry</h2>
          
          <p class="text-gray-300 mb-4">
            We do not use any analytics or telemetry services that collect and store information about your use of the Service.
          </p>

          <h2 class="text-2xl font-bold text-white mt-8 mb-4">Third-Party Services</h2>
          
          <p class="text-gray-300 mb-4">
            Our Service uses third-party services that may collect information used to identify you:
          </p>
          <ul class="list-disc pl-6 mb-6 text-gray-300">
            <li class="mb-2"><strong>Clerk:</strong> For authentication services</li>
            <li class="mb-2"><strong>Supabase:</strong> For data storage</li>
            <li class="mb-2"><strong>Brave:</strong> For search suggestions</li>
            <li class="mb-2"><strong>Sentry:</strong> For error tracking</li>
          </ul>
          <p class="text-gray-300 mb-4">
            We encourage you to review the privacy policies of these third-party services.
          </p>

          <h2 class="text-2xl font-bold text-white mt-8 mb-4">Data Retention</h2>
          
          <p class="text-gray-300 mb-4">
            We will retain your personal information only for as long as necessary to fulfill the purposes outlined in this Privacy Policy. When your data is no longer needed for the purposes for which it was collected, we will delete or anonymize it.
          </p>

          <h2 class="text-2xl font-bold text-white mt-8 mb-4">Your Rights</h2>
          
          <p class="text-gray-300 mb-4">
            Depending on your location, you may have certain rights regarding your personal information, such as:
          </p>
          <ul class="list-disc pl-6 mb-6 text-gray-300">
            <li class="mb-2">The right to access personal information we hold about you</li>
            <li class="mb-2">The right to request correction or deletion of your personal information</li>
            <li class="mb-2">The right to restrict or object to our processing of your personal information</li>
            <li class="mb-2">The right to data portability</li>
          </ul>
          <p class="text-gray-300 mb-4">
            To exercise these rights, please contact us using the contact information provided below.
          </p>

          <h2 class="text-2xl font-bold text-white mt-8 mb-4">Changes to This Privacy Policy</h2>
          
          <p class="text-gray-300 mb-4">
            We may update our Privacy Policy from time to time. We will notify you of any changes by posting the new Privacy Policy on this page and updating the "Last updated" date.
          </p>

          <h2 class="text-2xl font-bold text-white mt-8 mb-4">Contact Us</h2>
          
          <p class="text-gray-300 mb-4">
            If you have any questions about this Privacy Policy, please contact us:
          </p>
          <ul class="list-disc pl-6 mb-6 text-gray-300">
            <li class="mb-2">By email: <a href="mailto:support@betternewtab.com" class="text-blue-400 hover:underline">support@betternewtab.com</a></li>
            <li class="mb-2">By visiting our <a href="/contact" class="text-blue-400 hover:underline">contact page</a></li>
          </ul>
        </div>
      </div>
    </div>

    <!-- Footer -->
    <div class="pt-24 text-center bottom-0 w-full">
      <footer
        class="px-4 py-4 min-h-24 flex align-center border-t border-gray-700 bg-white/5 backdrop-filter backdrop-blur-3xl relative z-10">
        <div class="container mx-auto">
          <div class="flex justify-between items-center">
            <!-- Company Name -->
            <div class="flex items-center space-x-2">
              <span class="text-white text-xl font-semibold">BetterNewTab_</span>
            </div>

            <!-- Footer Links -->
            <div class="hidden md:flex space-x-8 border border-gray-300 px-6 py-4 rounded-full">
              <a href="/docs/getting-started" class="text-gray-300 hover:text-white transition-colors">Getting Started</a>
              <span class="text-gray-300 cursor-default">•</span>
              <a href="/privacy-policy" class="text-gray-300 hover:text-white transition-colors">Privacy Policy</a>
              <span class="text-gray-300 cursor-default">•</span>
              <a href="/terms-of-service" class="text-gray-300 hover:text-white transition-colors">Terms of Service</a>
              <span class="text-gray-300 cursor-default">•</span>
              <a href="/contact" class="text-gray-300 hover:text-white transition-colors">Contact</a>
            </div>

            <!-- Social Media Links -->
            <div class="flex items-center space-x-4">
              <a href="#" class="text-gray-300 hover:text-white transition-colors">
                <v-icon icon="mdi-twitter" />
              </a>
              <a href="#" class="text-gray-300 hover:text-white transition-colors">
                <v-icon icon="mdi-github" />
              </a>
              <a href="#" class="text-gray-300 hover:text-white transition-colors">
                <v-icon icon="mdi-linkedin" />
              </a>
            </div>
          </div>
        </div>
      </footer>
    </div>

    <v-dialog v-model="showSignIn" max-width="600px">
      <div class="m-auto">
        <div id="sign-in"></div>
      </div>
    </v-dialog>
  </div>
</template>

<script setup lang="ts">
import { ref, nextTick, onMounted } from "vue";
import { Clerk } from "@clerk/clerk-js";
import { cache } from "@/utils/cache";

const clerkPubKey = import.meta.env.VITE_CLERK_PUBLISHABLE_KEY;
const clerk = new Clerk(clerkPubKey);
const showSignIn = ref(false);

const handleShowSignIn = () => {
  // Clear all cache before signing in to ensure a fresh state
  cache.clearAll();
  
  showSignIn.value = true;
  nextTick(() => {
    const signInDiv = document.getElementById("sign-in");
    if (signInDiv) {
      clerk.mountSignIn(signInDiv as HTMLDivElement);
    }
  });
};

const handleShowSignUp = () => {
  // Clear all cache before signing up to ensure a fresh state
  cache.clearAll();
  
  showSignIn.value = true;
  nextTick(() => {
    const signInDiv = document.getElementById("sign-in");
    if (signInDiv) {
      clerk.mountSignUp(signInDiv as HTMLDivElement);
    }
  });
};

onMounted(async () => {
  await clerk.load();
});
</script>

<style scoped>
.v-text-field :deep(.v-field__input) {
  color: white !important;
}

.v-text-field :deep(.v-field__outline) {
  --v-field-border-opacity: 0.2;
}

.animated-gradient {
  animation: animateBg 12s ease infinite;
  background-image: linear-gradient(45deg, rgba(255, 255, 255, 0.6), rgb(141, 145, 255));
  background-size: 400% 400%;
  opacity: 0.5;
}

@keyframes animateBg {
  0% {
    background-position: 0% 50%;
  }

  50% {
    background-position: 100% 50%;
  }

  100% {
    background-position: 0% 50%;
  }
}

.prose {
  max-width: 100%;
}

.prose h2 {
  color: white;
  margin-top: 1.5rem;
  margin-bottom: 1rem;
}

.prose p, .prose ul, .prose li {
  color: rgb(209, 213, 219);
}

.prose a {
  color: rgb(96, 165, 250);
  text-decoration: none;
}

.prose a:hover {
  text-decoration: underline;
}

div {
  white-space: normal !important;
  overflow: visible !important;
  text-overflow: clip !important;
}
</style>