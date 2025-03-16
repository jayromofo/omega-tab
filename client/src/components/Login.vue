<template>
    <v-dialog v-model="dialog" max-width="400px">
        <v-card class="rounded-lg">
            <v-card-title class="text-center pa-4">
                <span class="text-h5">Login</span>
            </v-card-title>
            
            <v-card-text>
                <v-form ref="form" v-model="valid" lazy-validation>
                    <v-text-field
                        v-model="email"
                        :rules="emailRules"
                        label="Email"
                        required
                        prepend-icon="mdi-email"
                    ></v-text-field>
                    
                    <v-text-field
                        v-model="password"
                        :rules="passwordRules"
                        label="Password"
                        type="password"
                        required
                        prepend-icon="mdi-lock"
                    ></v-text-field>
                    
                    <v-btn
                        block
                        color="primary"
                        class="mt-4"
                        :disabled="!valid"
                        @click="login"
                    >
                        Login
                    </v-btn>
                </v-form>
            </v-card-text>
            
            <v-divider></v-divider>
            
            <v-card-text class="text-center">
                <div class="text-body-2 mb-3">Or continue with</div>
                
                <div class="d-flex justify-space-around">
                    <v-btn icon color="red" variant="outlined" @click="loginWithGoogle">
                        <v-icon>mdi-google</v-icon>
                    </v-btn>
                    
                    <v-btn icon color="blue" variant="outlined" @click="loginWithMicrosoft">
                        <v-icon>mdi-microsoft</v-icon>
                    </v-btn>
                    
                    <v-btn icon color="black" variant="outlined" @click="loginWithGithub">
                        <v-icon>mdi-github</v-icon>
                    </v-btn>
                </div>
            </v-card-text>
        </v-card>
    </v-dialog>
</template>

<script>
export default {
    name: 'LoginModal',
    data() {
        return {
            dialog: true,
            valid: true,
            email: '',
            password: '',
            emailRules: [
                v => !!v || 'Email is required',
                v => /.+@.+\..+/.test(v) || 'Email must be valid',
            ],
            passwordRules: [
                v => !!v || 'Password is required',
                v => v.length >= 6 || 'Password must be at least 6 characters',
            ],
        }
    },
    methods: {
        open() {
            this.dialog = true
        },
        close() {
            this.dialog = false
        },
        login() {
            if (this.$refs.form.validate()) {
                // Implement login logic here
                console.log('Logging in with email and password')
            }
        },
        loginWithGoogle() {
            // Implement Google OAuth login
            console.log('Logging in with Google')
        },
        loginWithMicrosoft() {
            // Implement Microsoft OAuth login
            console.log('Logging in with Microsoft')
        },
        loginWithGithub() {
            // Implement GitHub OAuth login
            console.log('Logging in with GitHub')
        }
    }
}
</script>

<style scoped>
.v-card {
    border-radius: 12px;
}
</style>