<template>
  <div class="texture-uploader">
    <h2>Neue Texture erstellen</h2>

    <form @submit.prevent="submitTexture">
      <div class="field">
        <label for="skin_name">Name</label>
        <input id="skin_name" v-model="skinName" required />
      </div>

      <div class="field">
        <label for="texture_type">Typ</label>
        <select id="texture_type" v-model="textureType" required>
          <option value="Skin">Skin</option>
          <option value="Cape">Cape</option>
          <option value="Elytra">Elytra</option>
        </select>
      </div>

      <div class="field">
        <label for="file">PNG-Datei auswählen</label>
        <input id="file" type="file" accept="image/png" @change="handleFileChange" required />
      </div>

      <div v-if="preview" class="preview">
        <p>Vorschau:</p>
        <img :src="preview" alt="Preview" />
      </div>

      <button type="submit" :disabled="isSubmitting">
        {{ isSubmitting ? "Wird hochgeladen..." : "Hochladen" }}
      </button>
    </form>

    <p v-if="message" class="message">{{ message }}</p>
  </div>
</template>

<script setup lang="ts">
import { ref } from 'vue'

const skinName = ref('')
const textureType = ref('Skin')
const imageData = ref<string | undefined>(undefined)
const preview = ref<string | null>(null)
const message = ref('')
const isSubmitting = ref(false)

function handleFileChange(event: Event) {
  const file = (event.target as HTMLInputElement).files?.[0]
  if (!file) return

  const reader = new FileReader()
  reader.onload = () => {
    imageData.value = (reader.result as string).split(',')[1] // remove data:image/png;base64,
    preview.value = reader.result as string
  }
  reader.readAsDataURL(file)
}

async function submitTexture() {
  if (!imageData.value) {
    message.value = 'Bitte eine PNG-Datei auswählen.'
    return
  }

  isSubmitting.value = true
  message.value = ''

  const body = {
    skin_name: skinName.value,
    texture_type: textureType.value,
    image_data: imageData.value
  }

  try {
    const res = await fetch('/api/texture', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(body)
    })

    if (!res.ok) throw new Error(await res.text())

    const data = await res.json()
    message.value = `Texture "${data.skin_name}" erfolgreich erstellt!`
  } catch (err: any) {
    message.value = 'Fehler beim Hochladen: ' + err.message
  } finally {
    isSubmitting.value = false
  }
}
</script>

<style scoped>
.texture-uploader {
  max-width: 400px;
  margin: auto;
  display: flex;
  flex-direction: column;
  gap: 1rem;
}

.field {
  display: flex;
  flex-direction: column;
}

.preview img {
  max-width: 128px;
  image-rendering: pixelated;
  border: 1px solid #ccc;
  margin-top: 0.5rem;
}

.message {
  text-align: center;
  font-weight: bold;
}
</style>
