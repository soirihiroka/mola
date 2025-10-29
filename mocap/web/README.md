# Mola Mocap Tool Web

A web-based motion capture tool using SolidJS and MediaPipe for real-time pose, face, and hand tracking via your webcam.

## Features

* **Real-time Tracking**: Captures motion directly from your webcam.
* **Multi-Landmarker Support**:
  * Face Landmarking
  * Hand Landmarking (for both hands, with distinct colors)
  * Pose Landmarking
* **API Integration**: Send captured landmark data to a specified API endpoint.
* **Configurable**:
  * Toggle individual models (Face, Hand, Pose) on/off.
  * Select from available camera devices.
  * Set API URL for data transmission.
  * Adjust minimum confidence levels for detection.
* **Performance Metrics**: Displays current FPS and video input resolution.
* **Responsive UI**: Adapts to landscape and portrait orientations.
* **PWA Ready**: Includes a badge for Progressive Web App installation status.

## Tech Stack

* **Frontend**: SolidJS
* **Machine Learning**: MediaPipe Tasks Vision (@mediapipe/tasks-vision)
* **Language**: TypeScript
* **Styling**: Adwaita-like CSS (from `adw.css`)
* **UI Components**: Kobalte Core (Switch, Toast)

## Getting Started

1. **Clone the repository:**

    ```bash
    git clone https://github.com/yamada-sexta/mocap-tool-web.git
    cd mocap-tool-web
    ```

2. **Install dependencies:**

    ```bash
    npm install
    # or yarn install
    ```

3. **Run the development server:**

    ```bash
    npm run dev
    # or yarn dev
    ```

    The application will typically be available at `http://localhost:3000`.

## Usage

1. Allow camera access when prompted.
2. Configure the API URL in the "Connection" section if you intend to send data to an external service.
3. Enable the desired tracking models (Face, Hand, Pose) in the "Models" section.
4. The detected landmarks will be overlaid on the video feed and, if the API URL is valid and models are active, data will be sent to your endpoint.
