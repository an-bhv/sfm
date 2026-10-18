import init, { SfmPipeline } from "./pkg/sfm.js";

async function setup() {
    await init(); 

    const fileInput = document.getElementById('imageInput');

    const pipeline = new SfmPipeline();
    
    fileInput.addEventListener('change', async (event) => {
        const files = event.target.files;
        if (!files) return;

        for (const file of files) {
            const buffer = await file.arrayBuffer();
            pipeline.add_image(new Uint8Array(buffer));
          }

      pipeline.run_pipeline(); 
      
    });
}

setup();