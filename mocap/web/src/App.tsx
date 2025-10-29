import { Accessor, children, createEffect, createSignal, For, JSX, JSXElement, Setter, Show, type Component } from 'solid-js';
import { PoseLandmarker, FilesetResolver, FaceLandmarker, DrawingUtils, HandLandmarker, PoseLandmarkerResult, HandLandmarkerResult, FaceLandmarkerResult } from "@mediapipe/tasks-vision"
import { Switch } from '@kobalte/core/switch';
import CameraIcon from './assets/icons/camera-photo-symbolic.svg?component-solid';
import "./adw.css"
import Spinner from './components/spinner';
import { makePersisted } from '@solid-primitives/storage';
import { Portal } from 'solid-js/web';
import { Toast } from '@kobalte/core/toast';
import PWABadge from './components/pwa-badge';
import { OptionsUI } from './components/options-ui';
const App: Component = () => {
  const [isLandscape, setIsLandscape] = createSignal(window.innerWidth > window.innerHeight);
  const helperCanvas = (<canvas
    style={{
      "position": "absolute",
      "width": "100%",
      "height": "100%",
      "pointer-events": "none",
      "max-width": "100vw",     // Limit width to viewport
      "max-height": "100vh",    // Limit height to viewport
    }}
  ></canvas>) as HTMLCanvasElement;
  const ctx = helperCanvas.getContext("2d")!;
  const drawingUtils = new DrawingUtils(ctx);

  const [apiUrl, setApiUrl] = makePersisted(createSignal(""), { storage: localStorage, name: "apiUrl" });
  const [urlValid, setUrlValid] = createSignal(false);
  const [minConfidence, setMinConfidence] = createSignal(0.3);

  const [stream, setStream] = createSignal<MediaStream>();

  const [faceLandmarker, setFaceLandmarker] = createSignal<FaceLandmarker>();
  const [handLandmarker, setHandLandmarker] = createSignal<HandLandmarker>();
  const [poseLandmarker, setPoseLandmarker] = createSignal<PoseLandmarker>();
  const [showLoading, setShowLoading] = createSignal(true);

  createEffect(async () => {
    try {
      const url = apiUrl() + "/pair";
      const res = await fetch(url, {});
      // console.log("res", res);
      const json = await res.json();
      if (!json["is_mocap"]) {
        throw "Not mocap!"
      }
      setUrlValid(true);
    } catch (e) {
      console.log("Error with the URL", e);
      setUrlValid(false);
    }
  });

  createEffect(() => {
    function handleResize() {
      setIsLandscape(window.innerWidth > window.innerHeight);
    }
    window.addEventListener("resize", handleResize);
    handleResize();
    return () => window.removeEventListener("resize", handleResize);
  });

  createEffect(() => {
    if (faceLandmarker() && handLandmarker() && poseLandmarker())
      setShowLoading(false);
  })

  const [enableFaceLandmarker, setUseEnableFaceLandmarker] = makePersisted(createSignal(false), { storage: localStorage, name: "enableFaceLandmarker" });
  const [enablePoseLandmarker, setUseEnablePoseLandmarker] = makePersisted(createSignal(false), { storage: localStorage, name: "enablePoseLandmarker" });
  const [enableHandLandmarker, setEnableHandLandmarker] = makePersisted(createSignal(false), { storage: localStorage, name: "enableHandLandmarker" });
  const [poseLandmarkerResult, setPoseLandmarkerResult] = createSignal<PoseLandmarkerResult>();
  const [handLandmarkerResult, setHandLandmarkerResult] = createSignal<HandLandmarkerResult>();
  const [faceLandmarkerResult, setFaceLandmarkerResult] = createSignal<FaceLandmarkerResult>();

  const [currentCamera, setCurrentCamera] = createSignal("");

  async function getCameras() {
    const devices = await navigator.mediaDevices.enumerateDevices();
    return devices.filter((device) => device.kind === "videoinput");
  }
  const [fps, setFps] = createSignal(-1);
  const [videoWidth, setVideoWidth] = createSignal(-1);
  const [videoHeight, setVideoHeight] = createSignal(-1);
  const [videoRatio, setVideoRatio] = createSignal(1);

  const [cameras, setCameras] = createSignal<MediaDeviceInfo[]>([]);

  createEffect(async () => {
    const cameras = await getCameras();
    setCameras(cameras);
    if (cameras.length > 0) {
      setCurrentCamera(cameras[0].deviceId);
    }
  });

  createEffect(() => {
    setVideoRatio(videoWidth() / videoHeight())
    helperCanvas.style.width = videoWidth() + "px",
      helperCanvas.style.height = videoHeight() + "px"

  })

  createEffect(() => {
    console.log("API URL", apiUrl());
  })

  async function accessCamera() {
    try {
      const s = await navigator.mediaDevices
        .getUserMedia({ video: true });
      setStream(s);
    } catch (e: any) {
      console.error("Error while trying to access camera", e.name, e.message);
    }
  }
  accessCamera();

  let videoRef!: HTMLVideoElement;

  createEffect(() => {
    const s = stream();
    if (s && videoRef)
      videoRef.srcObject = s;
  });

  let lastVideoTime = -1;
  let frameCount = 0;
  let lastFpsUpdateTime = performance.now();

  async function predictWebcam() {
    const video = videoRef;
    const fL = faceLandmarker();
    const hL = handLandmarker();
    const pL = poseLandmarker();
    if (!fL || !hL || !pL) {
      console.log("Face Landmarker isn't ready!");
      window.requestAnimationFrame(predictWebcam);
      return;
    };

    // Increment frame count
    frameCount++;

    let now = performance.now();
    if (now - lastFpsUpdateTime >= 1000) {
      setFps(frameCount)
      frameCount = 0;
      lastFpsUpdateTime = now;
    }

    if (video.currentTime !== lastVideoTime) {
      helperCanvas.style.width = video.clientWidth + "px";
      helperCanvas.style.height = video.clientHeight + "px";
      helperCanvas.width = videoWidth();
      helperCanvas.height = videoHeight();

      lastVideoTime = video.currentTime;

      if (enableFaceLandmarker()) {
        const faceRes = fL.detectForVideo(video, now);
        for (const landmarks of faceRes.faceLandmarks) {
          drawingUtils.drawConnectors(
            landmarks,
            FaceLandmarker.FACE_LANDMARKS_TESSELATION,
            { color: "#C0C0C070", lineWidth: 1 }
          );
          drawingUtils.drawConnectors(
            landmarks,
            FaceLandmarker.FACE_LANDMARKS_RIGHT_EYE,
            { color: "#FF3030" }
          );
          drawingUtils.drawConnectors(
            landmarks,
            FaceLandmarker.FACE_LANDMARKS_RIGHT_EYEBROW,
            { color: "#FF3030" }
          );
          drawingUtils.drawConnectors(
            landmarks,
            FaceLandmarker.FACE_LANDMARKS_LEFT_EYE,
            { color: "#30FF30" }
          );
          drawingUtils.drawConnectors(
            landmarks,
            FaceLandmarker.FACE_LANDMARKS_LEFT_EYEBROW,
            { color: "#30FF30" }
          );
          drawingUtils.drawConnectors(
            landmarks,
            FaceLandmarker.FACE_LANDMARKS_FACE_OVAL,
            { color: "#E0E0E0" }
          );
          drawingUtils.drawConnectors(landmarks, FaceLandmarker.FACE_LANDMARKS_LIPS, {
            color: "#E0E0E0"
          });
          drawingUtils.drawConnectors(
            landmarks,
            FaceLandmarker.FACE_LANDMARKS_RIGHT_IRIS,
            { color: "#FF3030" }
          );
          drawingUtils.drawConnectors(
            landmarks,
            FaceLandmarker.FACE_LANDMARKS_LEFT_IRIS,
            { color: "#30FF30" }
          );
        }
        setFaceLandmarkerResult(faceRes)
      }
      if (enableHandLandmarker()) {
        const handsRes = hL.detectForVideo(video, now);
        // Draw hand landmarks with different colors for left and right hands
        if (handsRes.landmarks && handsRes.handedness) {
          for (let i = 0; i < handsRes.landmarks.length; i++) {
            const landmarks = handsRes.landmarks[i];
            const handedness = handsRes.handedness[i][0].categoryName; // Get the handedness (e.g., 'Left', 'Right')

            // Define colors based on handedness
            const lineColor = handedness === 'Left' ? "#00FF00" : "#0000FF"; // Green for Left, Blue for Right
            const pointColor = handedness === 'Left' ? "#FF0000" : "#FFFF00"; // Red for Left, Yellow for Right

            // Draw connectors
            drawingUtils.drawConnectors(landmarks, HandLandmarker.HAND_CONNECTIONS, {
              color: lineColor,
              lineWidth: 5
            });
            // Draw landmarks
            drawingUtils.drawLandmarks(landmarks, { color: pointColor, lineWidth: 1 });
          }
        }
        setHandLandmarkerResult(handsRes);
      } else {
        setHandLandmarkerResult(undefined);
      }

      if (enablePoseLandmarker()) {
        const poseRes = pL.detectForVideo(video, now);

        setPoseLandmarkerResult(poseRes);
        for (const landmark of poseRes.landmarks) {
          drawingUtils.drawLandmarks(landmark, {
            radius: (data) => DrawingUtils.lerp(data.from!.z, -0.15, 0.1, 5, 1)
          });
          drawingUtils.drawConnectors(landmark, PoseLandmarker.POSE_CONNECTIONS);
        }
      } else {
        setPoseLandmarkerResult(undefined);
      }
    }
    // lastTime = now;
    window.requestAnimationFrame(predictWebcam);
  }

  createEffect(async () => {
    const poseRes = poseLandmarkerResult();
    if (poseRes && urlValid()) {
      try {
        const response = await fetch(apiUrl() + "/set_pose", {
          method: 'POST', // Specify the method
          headers: {
            'Content-Type': 'application/json', // Indicate that the body is JSON
          },
          body: JSON.stringify({ poseLandmarkerResult: poseRes }), // Convert the JavaScript object to a JSON string
        });

        // Check if the request was successful (status code 2xx)
        if (response.ok) {
          // console.log('Pose data sent successfully!');
        } else {
          console.error(`Failed to send pose data. Status: ${response.status}`);
          const errorText = await response.text();
          console.error('Error details:', errorText);
        }
      } catch (error) {
        console.error('An error occurred while sending pose data:', error);
      }
    }
  });

  createEffect(async () => {
    const handRes = handLandmarkerResult();

    // console.log("HandRes", handRes);
    if (handRes && urlValid()) {
      try {

        const response = await fetch(apiUrl() + "/set_hands", {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify({ handLandmarkerResult: handRes }),
        });
        if (response.ok) {
          // console.log('Hand data sent successfully!');
        } else {
          console.error(`Failed to send hand data. Status: ${response.status}`);
        }
      } catch (error) {
        console.error('An error occurred while sending hand data:', error);
      }
    }
  });

  createEffect(async () => {
    const faceRes = faceLandmarkerResult()
    console.log(faceRes)
    if (faceRes && urlValid()) {
      try {
        const response = await fetch(apiUrl() + "/set_face", {
          method: "POST",
          headers: {
            "Content-Type": "application/json",
          },
          body: JSON.stringify({ faceLandmarkerResult: faceRes })
        })
        if (response.ok) {
          // console.log('Hand data sent successfully!');
        } else {
          console.error(`Failed to send face data. Status: ${response.status}`, response);
        }
      } catch (error) {
        console.error('An error occurred while sending face data:', error);
      }
    }
  })


  async function createModels() {

    const vision = await FilesetResolver.forVisionTasks(
      "https://cdn.jsdelivr.net/npm/@mediapipe/tasks-vision/wasm"
    );

    console.log("filesetResolver", vision);
    const faceLandmarker = await FaceLandmarker.createFromOptions(vision, {
      baseOptions: {
        modelAssetPath: "https://storage.googleapis.com/mediapipe-models/face_landmarker/face_landmarker/float16/1/face_landmarker.task",
        delegate: "GPU"
      },
      outputFaceBlendshapes: true,
      runningMode: "VIDEO",
      numFaces: 1,
      minFaceDetectionConfidence: minConfidence(),
      minFacePresenceConfidence: minConfidence(),
      minTrackingConfidence: minConfidence()
    });

    console.log("Face Landmarker", faceLandmarker);
    setFaceLandmarker(faceLandmarker);

    const handLandmarker = await HandLandmarker.createFromOptions(vision, {
      baseOptions: {
        modelAssetPath: "https://storage.googleapis.com/mediapipe-models/hand_landmarker/hand_landmarker/float16/1/hand_landmarker.task",
        delegate: "GPU",
      },
      numHands: 2,
      runningMode: "VIDEO",
      minTrackingConfidence: minConfidence(),
      minHandDetectionConfidence: minConfidence(),
      minHandPresenceConfidence: minConfidence()
    });

    setHandLandmarker(handLandmarker);

    const poseLandmarker = await PoseLandmarker.createFromOptions(vision, {
      baseOptions: {
        modelAssetPath: `https://storage.googleapis.com/mediapipe-models/pose_landmarker/pose_landmarker_heavy/float16/latest/pose_landmarker_heavy.task`,
        delegate: "GPU"
      },
      runningMode: "VIDEO",
      minPoseDetectionConfidence: minConfidence(),
      minPosePresenceConfidence: minConfidence(),
      minTrackingConfidence: minConfidence(),
    });
    setPoseLandmarker(poseLandmarker);

  };

  function Toggle(props: { l: string, checked: Accessor<boolean>, onChange: Setter<boolean> }) {
    return <div
      style={{ display: "flex", gap: "8px", "align-items": 'center', width: "100%", "justify-content": "space-between" }}
    >
      <label>{props.l}</label>
      <Switch class="switch"
        checked={props.checked()}
        onChange={props.onChange}
      >
        <Switch.Input class="switch__input" />
        <Switch.Control class="switch__control">
          <Switch.Thumb class="switch__thumb" />
        </Switch.Control>
      </Switch>
    </div>
  }

  function List(props: { children?: JSX.Element[] | JSX.Element }) {
    return <Show
      when={props.children && Array.isArray(props.children)}
      fallback={
        <div style={{
          padding: "16px"
        }}>
          {props.children}
        </div>
      }
    >
      <For
        each={props.children as JSX.Element[]}
      >
        {(element, i) =>
          <>
            <Show when={i() !== 0}>
              <div style={{ background: "var(--light-2)", width: "100%", height: "1px" }}></div>
            </Show>
            <div
              style={{
                padding: "16px"
              }}
            >
              {element}
            </div>
          </>
        }
      </For>
    </Show>
  }

  function Card(props: { children?: JSX.Element[] | JSX.Element, label: string }) {
    return <div
      style={{
        display: "flex", "flex-direction": "column", gap: "8px"
      }}
    >
      <label style={{ "font-weight": 600 }}>{props.label}</label>
      <div class='card' >
        {props.children}
      </div>
    </div>
  };


  createModels();
  return (
    <div style={{
      display: "flex",
      "flex-direction": isLandscape() ? "row" : "column"
    }}>
      <div
        style={{
          position: "relative",
          "display": "flex",
          "min-width": 0,
          width: isLandscape() ? "100%" : "100%",
          height: isLandscape() ? "auto" : "300px",
          "align-items": "center",
          "justify-content": "center"
        }}
      >
        <video
          style={{
            display: "block",
            "min-width": 0, "min-height": 0,
            width: isLandscape() ? "100%" : "auto",
            height: isLandscape() ? "auto" : "100%",
            "max-width": "100vw",     // Limit width to viewport
            "max-height": "100vh",    // Limit height to viewport
          }}
          ref={videoRef}
          on:loadeddata={(e) => {
            console.log("e", e);
            videoRef.play();
            predictWebcam();
            setVideoWidth(videoRef.videoWidth);
            setVideoHeight(videoRef.videoHeight);
          }}
          autoplay
        ></video>
        {helperCanvas}
        <Show
          when={faceLandmarker() && !stream()}
        >
          <div
            style={{
              "width": "100%",
              "height": "100%",
              display: "flex",
              "align-items": "center",
              "padding": "16px",
              "justify-content": "center"
            }}
          >
            <button
              class='button primary pill'
              onClick={accessCamera}
            >
              <CameraIcon fill='white'></CameraIcon>
              Access User Media
            </button>
          </div>

        </Show>
      </div>
      <div
        style={{
          width: isLandscape() ? "400px" : "100%",
          display: "flex",
          "flex-direction": "column",
          gap: "24px",
          padding: "16px",
          "box-sizing": "border-box",
          overflow: "auto", // Enable scrolling
          "max-height": "100vh", // Prevent overflow beyond viewport height
        }}

      >
        <Card
          label='Info'
        >
          <List>
            <label>FPS: <code>{fps()}</code></label>
            <label>Input: <code>{videoWidth()}x{videoHeight()}</code></label>
            <select
              onchange={async (e) => {
                const deviceId = e.target.value;
                const media = await navigator.mediaDevices.getUserMedia({
                  video: {
                    deviceId: deviceId,
                  }
                })
                setStream(media);
              }}
            >
              {cameras().map((camera) =>
                <option value={camera.deviceId}>{camera.label}</option>
              )}
            </select>

          </List>
        </Card>
        <Card
          label='Connection'
        >
          <List>
            <div style={{ "box-sizing": "border-box" }}>
              <label>API URL ({urlValid() ? "Valid" : "Invalid"})</label>
              <input class='adw' style={{ "width": "100%", "box-sizing": "border-box" }} value={apiUrl()} onchange={(e) => {
                setApiUrl(e.target.value)
              }}></input>
            </div>
          </List>
        </Card>
        <Card
          label='Models'
        >
          <Show when={!showLoading()} fallback={
            <div
              style={{
                width: "100%", "align-items": "center", display: "flex", "justify-content": "center", "padding-top": "8px", "padding-bottom": "8px", gap: "8px"
              }}
            >
              <Spinner size={40} strokeWidth={6}></Spinner>
              Loading Models...
            </div>

          }>
            <List>
              <Toggle l='Face' checked={enableFaceLandmarker} onChange={setUseEnableFaceLandmarker} />
              <Toggle l='Hand' checked={enableHandLandmarker} onChange={setEnableHandLandmarker} />
              <Toggle l='Pose' checked={enablePoseLandmarker} onChange={setUseEnablePoseLandmarker} />
            </List>
          </Show>
        </Card>
      </div>

      <Portal>
        <Toast.Region swipeDirection="down">
          <Toast.List class="toast__list" />
        </Toast.Region>
      </Portal>
      <PWABadge />
    </div>
  );
};

export default App;
